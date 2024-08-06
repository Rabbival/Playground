use crate::{get_mut_entity_else_return, prelude::*};

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
    mut add_timer_event_writer: EventWriter<AddTimerToEntity<Vec3>>,
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
                add_timer_event_writer.send(AddTimerToEntity {
                    timer: CustomTimer::<Vec3>::new(
                        TimeMultiplierId::GameTimeMultiplier,
                        *duration,
                        TimerValueCalculator::new(
                            *origin,
                            *target,
                            Interpolator {
                                power: ORB_COLLECTION_POWER,
                            },
                        ),
                        Some(EventFromTimerType::MoveInDirectLine),
                        *once_done,
                    ),
                    attach_to: *entity,
                });
            }
        }
    }
}

fn listen_for_translation_update_requests(
    mut event_reader: EventReader<EventFromTimer<Vec3>>,
    mut transforms: Query<&mut Transform>,
) {
    for event_from_timer in event_reader.read() {
        if let Some(EventFromTimerType::MoveInDirectLine) =
            event_from_timer.try_get_as_going_event()
        {
            update_entity_translation(
                event_from_timer.entity(),
                &mut transforms,
                event_from_timer.current_value(),
            );
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
