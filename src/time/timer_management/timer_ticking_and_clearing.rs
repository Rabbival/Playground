use crate::{plugin_for_implementors_of_trait, prelude::*};

plugin_for_implementors_of_trait!(TimerTickingAndClearingPlugin, Numeric);

impl<T: Numeric> Plugin for TimerTickingAndClearingPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (tick_once_done_timers, tick_full_timers::<T>).in_set(TimerSystemSet::TimerTicking),
                (clear_once_done_timers, clear_full_timers::<T>)
                    .in_set(EndOfFrameSystemSet::TimerClearing),
            ),
        );
    }
}

fn tick_once_done_timers(
    mut timer_done_event_writer: EventWriter<TimerDoneEvent>,
    mut timers: Query<(&mut OnceDoneTimer, Entity)>,
    time_multipliers: Query<&TimeMultiplier>,
    time: Res<Time>,
) {
    let time_delta = time.delta_seconds();
    for (mut timer, timer_entity) in &mut timers {
        tick_once_done_timer_and_send_event(
            &mut timer_done_event_writer,
            time_delta * calculate_time_multiplier(&time_multipliers, timer.time_multipliers),
            &mut timer,
            timer_entity,
        );
    }
}

fn tick_once_done_timer_and_send_event(
    timer_done_event_writer: &mut EventWriter<TimerDoneEvent>,
    time_to_tick: f32,
    timer: &mut OnceDoneTimer,
    timer_entity: Entity,
) {
    if let Some(timer_done_event) = timer.tick_and_get_event_if_finished(time_to_tick) {
        timer_done_event_writer.send(TimerDoneEvent {
            event_type: timer_done_event,
            affected_entities: timer.affected_entities,
            timer_entity,
        });
    }
}

pub fn tick_full_timers<T: Numeric>(
    mut timer_going_event_writer: EventWriter<TimerGoingEvent<T>>,
    mut timer_done_event_writer: EventWriter<TimerDoneEvent>,
    mut timers: Query<(&mut FullTimer, &ValueByInterpolation<T>, Entity)>,
    time_multipliers: Query<&TimeMultiplier>,
    time: Res<Time>,
) {
    let time_delta = time.delta_seconds();
    for (mut timer, value_calculator, timer_entity) in &mut timers {
        tick_full_timer_and_send_events(
            &mut timer_going_event_writer,
            &mut timer_done_event_writer,
            time_delta * calculate_time_multiplier(&time_multipliers, timer.time_multipliers),
            &mut timer,
            value_calculator,
            timer_entity,
        );
    }
}

fn tick_full_timer_and_send_events<T: Numeric>(
    timer_going_event_writer: &mut EventWriter<TimerGoingEvent<T>>,
    timer_done_event_writer: &mut EventWriter<TimerDoneEvent>,
    time_to_tick: f32,
    timer: &mut FullTimer,
    value_calculator: &ValueByInterpolation<T>,
    timer_entity: Entity,
) {
    if let Some(normalized_progress) = timer.tick_and_get_normalized_progress(time_to_tick) {
        timer_going_event_writer.send(TimerGoingEvent::<T> {
            event_type: timer.send_as_going,
            entities: timer.affected_entities,
            value: value_calculator.calculate_current_value(normalized_progress),
        });
        if timer.finished() {
            timer_done_event_writer.send(TimerDoneEvent {
                event_type: timer.send_once_done,
                affected_entities: timer.affected_entities,
                timer_entity,
            });
        }
    }
}

fn calculate_time_multiplier<const N: usize>(
    time_multipliers: &Query<&TimeMultiplier>,
    multipliers_timer_subscribes_to: VecBasedArray<TimeMultiplierId, N>,
) -> f32 {
    let mut calculated_multiplier = DEFAULT_TIME_MULTIPLIER;
    for multiplier_id_from_timer in multipliers_timer_subscribes_to.iter() {
        for time_multiplier in time_multipliers {
            if time_multiplier.id() == multiplier_id_from_timer {
                calculated_multiplier *= time_multiplier.value();
            }
        }
    }
    calculated_multiplier
}

fn clear_once_done_timers(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    once_done_timers: Query<Entity, With<OnceDoneTimer>>,
    mut commands: Commands,
) {
    for timer_done_event in timer_done_event_reader.read() {
        if let Ok(entity) = once_done_timers.get(timer_done_event.timer_entity) {
            match commands.get_entity(entity) {
                Some(mut entity_commands) => {
                    entity_commands.remove::<OnceDoneTimer>();
                }
                None => {
                    print_error(
                        EntityError::CommandsCouldntGetEntity("OnceDoneTimer"),
                        vec![LogCategory::RequestNotFulfilled],
                    );
                }
            }
        }
    }
}

fn clear_full_timers<T: Numeric>(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    full_timers: Query<Entity, (With<FullTimer>, With<ValueByInterpolation<T>>)>,
    mut commands: Commands,
) {
    for timer_done_event in timer_done_event_reader.read() {
        if let Ok(entity) = full_timers.get(timer_done_event.timer_entity) {
            match commands.get_entity(entity) {
                Some(mut entity_commands) => {
                    entity_commands.remove::<CalculatingTimer<T>>();
                }
                None => {
                    print_error(
                        EntityError::CommandsCouldntGetEntity("CalculatingTimer"),
                        vec![LogCategory::RequestNotFulfilled],
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_timer_clearing() {
        let mut app = App::new();
        app.init_resource::<Time>()
            .add_event::<TimerDoneEvent>()
            .add_systems(
                Update,
                (tick_once_done_timers, clear_once_done_timers).chain(),
            );

        add_timer_and_advance_time(&mut app);
        app.update();

        assert_eq!(
            app.world_mut()
                .query::<&OnceDoneTimer>()
                .iter(app.world())
                .len(),
            0
        );
    }

    fn add_timer_and_advance_time(app: &mut App) {
        app.world_mut().commands().spawn(OnceDoneTimer::new(
            vec![],
            vec![],
            A_MILLISECOND_IN_SECONDS,
            TimerDoneEventType::default(),
        ));
        app.world_mut()
            .resource_mut::<Time>()
            .as_mut()
            .advance_by(Duration::from_secs_f32(A_MILLISECOND_IN_SECONDS));
    }
}
