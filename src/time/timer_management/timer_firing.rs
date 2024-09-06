use crate::{plugin_for_implementors_of_trait, prelude::*};

plugin_for_implementors_of_trait!(TimerFiringPlugin, Numeric);

impl<T: Numeric> Plugin for TimerFiringPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_emitting_timer_firing_requests::<T>
                .in_set(TimerSystemSet::PreTickingPreperations),
        );
    }
}

pub fn listen_for_emitting_timer_firing_requests<T: Numeric>(
    mut event_reader: EventReader<TimerFireRequest>,
    mut remove_from_timer_entities_writer: EventWriter<RemoveFromTimerAffectedEntities>,
    mut affected_by_timer_query: Query<&mut AffectingTimerCalculators>,
    timer_calculators: Query<&GoingEventValueCalculator<T>>,
    emitting_timers: Query<&EmittingTimer>,
    mut commands: Commands,
) {
    for timer_fire_request in event_reader.read() {
        let newborn_timer = commands.spawn(timer_fire_request.0).id();
        for timer_affected_entity in timer_fire_request.0.affected_entities.iter() {
            if let Some(value_calculator_entity) = timer_affected_entity.value_calculator_entity {
                if let Ok(value_calculator) = timer_calculators.get(value_calculator_entity) {
                    match affected_by_timer_query.get_mut(timer_affected_entity.affected_entity) {
                        Ok(mut affecting_timer_calculators) => {
                            set_active_calculator_and_destroy_inactive(
                                &mut remove_from_timer_entities_writer,
                                &mut affecting_timer_calculators,
                                TimerAndCalculator {
                                    timer: newborn_timer,
                                    value_calculator: value_calculator_entity,
                                },
                                value_calculator,
                                &emitting_timers,
                                &mut commands,
                            );
                        }
                        Err(_) => print_warning(
                            EntityError::EntityNotInQuery(String::from(
                                "couldn't find entity in affecting timers component query",
                            )),
                            vec![LogCategory::RequestNotFulfilled, LogCategory::Time],
                        ),
                    }
                }
            }
        }
    }
}

fn set_active_calculator_and_destroy_inactive<T: Numeric>(
    remove_from_timer_entities_writer: &mut EventWriter<RemoveFromTimerAffectedEntities>,
    affecting_timer_calculators: &mut AffectingTimerCalculators,
    newborn_timer_and_calculator: TimerAndCalculator,
    value_calculator: &GoingEventValueCalculator<T>,
    emitting_timers: &Query<&EmittingTimer>,
    commands: &mut Commands,
) {
    let maybe_existing_affecting_calculator = affecting_timer_calculators.insert(
        value_calculator.going_event_type(),
        newborn_timer_and_calculator,
        value_calculator.set_policy,
    );
    if let Some(existing_calculator_entity) = maybe_existing_affecting_calculator {
        let maybe_timer_to_remove_from = match value_calculator.set_policy {
            TimerCalculatorSetPolicy::AlwaysTakeNew => Some(existing_calculator_entity),
            TimerCalculatorSetPolicy::IgnoreNewIfAssigned => Some(newborn_timer_and_calculator),
        };
        if let Some(timer_to_remove_from) = maybe_timer_to_remove_from {
            destory_inactive_and_send_removal_request(
                remove_from_timer_entities_writer,
                timer_to_remove_from,
                emitting_timers,
                commands,
            );
        }
    }
}

fn destory_inactive_and_send_removal_request(
    remove_from_timer_entities_writer: &mut EventWriter<RemoveFromTimerAffectedEntities>,
    remove_from_and_destroy: TimerAndCalculator,
    emitting_timers: &Query<&EmittingTimer>,
    commands: &mut Commands,
) {
    commands
        .entity(remove_from_and_destroy.value_calculator)
        .despawn();
    if let Ok(timer) = emitting_timers.get(remove_from_and_destroy.timer) {
        if let Some(affected_entity) = timer
            .affected_entities
            .get_by_calculator_entity(remove_from_and_destroy.value_calculator)
        {
            remove_from_timer_entities_writer.send(RemoveFromTimerAffectedEntities {
                timer_entity: remove_from_and_destroy.timer,
                entity_to_remove: affected_entity,
            });
        }
    }
}
