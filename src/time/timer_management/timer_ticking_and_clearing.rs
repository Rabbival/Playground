use crate::{plugin_for_implementors_of_trait, prelude::*};

plugin_for_implementors_of_trait!(TimerTickingAndClearingGenericPlugin, Numeric);
pub struct TimerTickingAndClearingPlugin;

impl<T: Numeric> Plugin for TimerTickingAndClearingGenericPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            calculate_value_and_send_going_event::<T>.in_set(TimerSystemSet::PostTickingImmidiate),
        );
    }
}

impl Plugin for TimerTickingAndClearingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (tick_once_done_timers, tick_full_timers).in_set(TimerSystemSet::TimerTicking),
                clear_timers.in_set(EndOfFrameSystemSet::TimerClearing),
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

pub fn tick_full_timers(
    mut calculation_requests_writer: EventWriter<CalculateAndSendGoingEvent>,
    mut timer_done_event_writer: EventWriter<TimerDoneEvent>,
    mut timers: Query<(&mut FullTimer, Entity)>,
    time_multipliers: Query<&TimeMultiplier>,
    time: Res<Time>,
) {
    let time_delta = time.delta_seconds();
    for (mut timer, timer_entity) in &mut timers {
        let modified_time_delta =
            time_delta * calculate_time_multiplier(&time_multipliers, timer.time_multipliers);
        tick_full_timer_and_send_events(
            &mut calculation_requests_writer,
            &mut timer_done_event_writer,
            modified_time_delta,
            &mut timer,
            timer_entity,
        );
    }
}

fn tick_full_timer_and_send_events(
    calculation_requests_writer: &mut EventWriter<CalculateAndSendGoingEvent>,
    timer_done_event_writer: &mut EventWriter<TimerDoneEvent>,
    time_to_tick: f32,
    timer: &mut FullTimer,
    timer_entity: Entity,
) {
    if let Some(normalized_progress) = timer.tick_and_get_normalized_progress(time_to_tick) {
        calculation_requests_writer.send(CalculateAndSendGoingEvent {
            affected_entities: timer.affected_entities,
            normalized_progress,
            event_type_to_send: timer.send_as_going,
        });
        if timer.finished() {
            timer_done_event_writer.send(TimerDoneEvent {
                event_type: timer.send_once_done,
                affected_entities: timer.affected_entities_without_interpolators(),
                timer_entity,
            });
        }
    }
}

pub fn calculate_value_and_send_going_event<T: Numeric>(
    mut calculation_requests_reader: EventReader<CalculateAndSendGoingEvent>,
    mut timer_going_event_writer: EventWriter<TimerGoingEvent<T>>,
    value_calculators: Query<&ValueByInterpolation<T>>,
) {
    for calculation_request in calculation_requests_reader.read() {
        for affected_entity in calculation_request.affected_entities.iter() {
            if let Ok(value_calculator) =
                value_calculators.get(affected_entity.value_calculator_entity)
            {
                let calculated_value = value_calculator
                    .calculate_current_value(calculation_request.normalized_progress);
                timer_going_event_writer.send(TimerGoingEvent::<T> {
                    event_type: calculation_request.event_type_to_send,
                    entity: affected_entity.affected_entity,
                    value: calculated_value,
                });
            }
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

fn clear_timers(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    once_done_timers: Query<Entity, With<OnceDoneTimer>>,
    full_timers: Query<(Entity, &FullTimer)>,
    mut commands: Commands,
) {
    for timer_done_event in timer_done_event_reader.read() {
        if let Ok(entity) = once_done_timers.get(timer_done_event.timer_entity) {
            despawn_entity_notify_on_fail(entity, "OnceDoneTimer", &mut commands);
        }
        if let Ok((entity, full_timer)) = full_timers.get(timer_done_event.timer_entity) {
            despawn_entity_notify_on_fail(entity, "FullTimer", &mut commands);
            for value_calculator_entity in full_timer.affected_entities_interpolators_only().iter()
            {
                despawn_entity_notify_on_fail(
                    value_calculator_entity,
                    "Interpolator from FullTimer",
                    &mut commands,
                );
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
            .add_systems(Update, (tick_once_done_timers, clear_timers).chain());

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
