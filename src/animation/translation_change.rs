use crate::{get_mut_entity_else_return, prelude::*, read_two_field_variant};

pub struct TranslationChangePlugin;

impl Plugin for TranslationChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                listen_for_init_translation_change_request.in_set(AnimationSystemSet::PreTicking),
                listen_for_translation_update_requests.in_set(AnimationSystemSet::PostTicking),
            ),
        );
    }
}

fn listen_for_init_translation_change_request(
    mut event_reader: EventReader<TranslationEventChannel>,
    mut time_event_writer: EventWriter<TimeEventChannel<Vec3>>,
) {
    for translation_event in event_reader.read() {
        match translation_event {
            TranslationEventChannel::MoveInDirectLine {
                entity,
                origin,
                target,
                duration,
                once_done,
            } => {
                time_event_writer.send(TimeEventChannel::AddTimerToEntity(
                    *entity,
                    CustomTimer::<Vec3>::new(
                        TimeMultiplierId::GameTimeMultiplier,
                        *duration,
                        TimerValueCalculator::new(
                            *origin,
                            *target,
                            MathFunction::Parabolic {
                                power: ORB_COLLECTION_POWER,
                            },
                        ),
                        Some(EventFromTimerType::MoveInDirectLine),
                        *once_done,
                    ),
                ));
            }
        }
    }
}

fn listen_for_translation_update_requests(
    mut event_reader: EventReader<TimeEventChannel<Vec3>>,
    mut transforms: Query<&mut Transform>,
) {
    for (&entity, event_from_timer) in
        read_two_field_variant!(event_reader, TimeEventChannel::EventFromTimer)
    {
        if let Some(EventFromTimerType::MoveInDirectLine) =
            event_from_timer.try_get_as_going_event()
        {
            update_entity_translation(entity, &mut transforms, event_from_timer.current_value());
        }
    }
}

fn update_entity_translation(
    entity: Entity,
    transforms: &mut Query<&mut Transform>,
    new_translation: Vec3,
) {
    let mut transform = get_mut_entity_else_return!(transforms, entity);
    transform.translation = new_translation;
}
