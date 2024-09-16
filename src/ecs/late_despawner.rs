use crate::prelude::*;

pub struct LateDespawnerPlugin;

impl Plugin for LateDespawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_despawn_requests_from_timers.in_set(EndOfFrameSystemSet::LateDespawn),
        );
    }
}

pub fn listen_for_despawn_requests_from_timers(
    mut event_reader: EventReader<TimerDoneEvent>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        if let TimerDoneEventType::DespawnAffectedEntities = event.event_type {
            for affected_entity in event.affected_entities.iter() {
                despawn_entity_notify_on_fail(
                    affected_entity.affected_entity,
                    "(affected entity from timer despawn affected entities request)",
                    &mut commands,
                );
            }
        }
    }
}
