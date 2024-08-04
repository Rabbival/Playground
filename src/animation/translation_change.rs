use crate::{get_mut_entity_else_return, prelude::*, read_single_field_variant};

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
    mut commands: Commands,
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
                commands.spawn(CustomTimer::<Vec3>::new(
                    TimeMultiplierId::GameTimeMultiplier,
                    *duration,
                    Some(*entity),
                    TimerValueCalculator::new(
                        *origin,
                        *target,
                        MathFunction::Parabolic {
                            power: ORB_COLLECTION_POWER,
                        },
                    ),
                    Some(EventFromTimerType::MoveInDirectLine),
                    *once_done,
                ));
            }
        }
    }
}

fn listen_for_translation_update_requests(
    mut event_reader: EventReader<TimerEventChannel<Vec3>>,
    mut transforms: Query<&mut Transform>,
    mut commands: Commands,
) {
    for event_from_timer in
        read_single_field_variant!(event_reader, TimerEventChannel::EventFromTimer)
    {
        if let Some(entity) = event_from_timer.try_get_relevant_entity() {
            if let Some(as_going_event) = event_from_timer.try_get_as_going_event() {
                if as_going_event == EventFromTimerType::MoveInDirectLine {
                    update_entity_translation(
                        entity,
                        &mut transforms,
                        event_from_timer.current_value(),
                    );
                }
                if let Some(_done_event) = event_from_timer.try_get_done_event() {
                    commands.entity(entity).despawn();
                }
            }
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
