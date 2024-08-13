use crate::prelude::*;

pub struct LateDespawnerPlugin;

impl Plugin for LateDespawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_for_despawn_requests_from_timers);
    }
}

pub fn listen_for_despawn_requests_from_timers(
    mut event_reader: EventReader<TimerDoneEvent>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        if let TimerDoneEventType::DespawnAffectedEntities = event.event_type {
            for entity in event.affected_entities.iter() {
                match commands.get_entity(entity) {
                    Some(mut entity_commands) => {
                        entity_commands.despawn();
                    }
                    None => {
                        print_error(
                            EntityError::CommandsCouldntGetEntity(
                                "(from timer despawn affected entities request)",
                            ),
                            vec![LogCategory::RequestNotFulfilled],
                        );
                    }
                }
            }
        }
    }
}
