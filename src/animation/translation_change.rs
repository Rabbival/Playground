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
    mut commands: Commands,
) {
    for translation_event in event_reader.read() {
        match translation_event {
            TranslationEventChannel::InitiateMoveInDirectLine {
                entity,
                origin,
                target,
                duration,
                once_done,
            } => {
                commands.spawn(CalculatingTimer {
                    timer: FullTimer::new(
                        vec![*entity],
                        vec![TimeMultiplierId::GameTimeMultiplier],
                        *duration,
                        TimerGoingEventType::Move(MoveEventFromTimer::InDirectLine),
                        *once_done,
                    ),
                    calculator: ValueByInterpolation::<Vec3>::new(
                        *origin,
                        *target,
                        Interpolator {
                            power: ORB_COLLECTION_POWER,
                        },
                    ),
                });
            }
        }
    }
}

fn listen_for_translation_update_requests(
    mut event_reader: EventReader<TimerGoingEvent<Vec3>>,
    mut transforms: Query<&mut Transform>,
) {
    for event_from_timer in event_reader.read() {
        if let TimerGoingEventType::Move(MoveEventFromTimer::InDirectLine) =
            event_from_timer.event_type
        {
            for entity in event_from_timer.entities.iter() {
                update_entity_translation(entity, &mut transforms, event_from_timer.value);
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
