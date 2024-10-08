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
    mut remove_from_timer_event_writer: EventWriter<RemoveFromTimerAffectedEntities>,
    affecting_timers_query: Query<&AffectingTimerCalculators>,
    parent_timer_sequence_query: Query<&TimerParentSequence>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        if let TimerDoneEventType::DespawnAffectedEntities(despawn_policy) = event.event_type {
            for affected_entity in event.affected_entities.iter() {
                match despawn_policy {
                    DespawnPolicy::DespawnSelf => {}
                    DespawnPolicy::DespawnSelfAndRemoveFromAffectingTimers => {
                        remove_from_all_affecting_entities(
                            &mut remove_from_timer_event_writer,
                            &affecting_timers_query,
                            affected_entity.affected_entity,
                        );
                    }
                    DespawnPolicy::DespawnSelfAndAffectingTimersAndParentSequences => {
                        destroy_affecting_timers_and_calculators_and_sequences(
                            affected_entity.affected_entity,
                            &affecting_timers_query,
                            &parent_timer_sequence_query,
                            &mut commands,
                        );
                    }
                }
                despawn_recursive_notify_on_fail(
                    affected_entity.affected_entity,
                    "(affected entity from timer despawn affected entities request)",
                    &mut commands,
                );
            }
        }
    }
}

fn remove_from_all_affecting_entities(
    remove_from_timer_event_writer: &mut EventWriter<RemoveFromTimerAffectedEntities>,
    affecting_timers_query: &Query<&AffectingTimerCalculators>,
    affected_entity: Entity,
) {
    if let Ok(affecting_timers) = affecting_timers_query.get(affected_entity) {
        for affecting_timers_of_type in affecting_timers.values() {
            for affecting_timer in affecting_timers_of_type {
                remove_from_timer_event_writer.send(RemoveFromTimerAffectedEntities {
                    timer_entity: affecting_timer.timer,
                    entity_to_remove: TimerAffectedEntity {
                        affected_entity,
                        value_calculator_entity: Some(affecting_timer.value_calculator),
                    },
                });
            }
        }
    } else {
        print_warning(
            format!(
                "Was asked to remove entity {:?} from affecting timers, but it has none.",
                affected_entity
            ),
            vec![LogCategory::RequestNotFulfilled],
        );
    }
}

fn destroy_affecting_timers_and_calculators_and_sequences(
    affected_entity: Entity,
    affecting_timers_query: &Query<&AffectingTimerCalculators>,
    parent_timer_sequence_query: &Query<&TimerParentSequence>,
    commands: &mut Commands,
) {
    if let Ok(affecting_timers) = affecting_timers_query.get(affected_entity) {
        for affecting_timers_of_type in affecting_timers.values() {
            for affecting_timer in affecting_timers_of_type {
                despawn_recursive_notify_on_fail(
                    affecting_timer.value_calculator,
                    "EmittingTimer",
                    commands,
                );
                if let Some(timer) = commands.get_entity(affecting_timer.timer) {
                    timer.despawn_recursive();
                    if let Ok(parent_sequence_component) =
                        parent_timer_sequence_query.get(affecting_timer.timer)
                    {
                        if let Some(timer_sequence) =
                            commands.get_entity(parent_sequence_component.parent_sequence)
                        {
                            timer_sequence.despawn_recursive();
                        }
                    }
                }
            }
        }
    } else {
        print_warning(
            format!(
                "Was asked to destroy the calculator, timer and sequence of entity {:?}, but it has no affecting timers component.",
                affected_entity
            ),
            vec![LogCategory::RequestNotFulfilled],
        );
    }
}
