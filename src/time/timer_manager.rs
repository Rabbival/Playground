use crate::prelude::*;

pub struct TimerManagerPlugin;

impl Plugin for TimerManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tick_timers::<f32>,
                tick_timers::<Vec2>,
                tick_timers::<Vec3>,
                tick_timers::<Quat>,
            )
                .in_set(TimerSystemSet::TimerTicking),
        );
    }
}

fn tick_timers<T: Numeric>(
    mut event_from_timer_writer: EventWriter<EventFromTimer<T>>,
    mut timers_not_on_multiplers: Query<(&mut CustomTimer<T>, Entity)>,
    time_multipliers: Query<&TimeMultiplier>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let time_delta = time.delta_seconds();
    for (mut timer, timer_entity) in &mut timers_not_on_multiplers {
        tick_and_send_timer_event(
            time_delta * get_time_multiplier(&time_multipliers, &timer),
            &mut timer,
            timer_entity,
            &mut event_from_timer_writer,
            &mut commands,
        );
    }
}

fn get_time_multiplier<T: Numeric>(
    time_multipliers: &Query<&TimeMultiplier>,
    timer: &CustomTimer<T>,
) -> f32 {
    for multiplier in time_multipliers {
        if timer.time_multiplier == multiplier.id() {
            return multiplier.value();
        }
    }
    print_warning(
        TimeRelatedError::TimeMultiplierNotFound(timer.time_multiplier),
        vec![LogCategory::RequestNotFulfilled, LogCategory::Time],
    );
    DEFAULT_TIME_MULTIPLIER
}

fn tick_and_send_timer_event<T: Numeric>(
    time_to_tick: f32,
    timer: &mut CustomTimer<T>,
    timer_entity: Entity,
    event_from_timer_writer: &mut EventWriter<EventFromTimer<T>>,
    commands: &mut Commands,
) {
    if let Some(partial_timer_event) = timer.tick_and_get_event(time_to_tick) {
        event_from_timer_writer.send(EventFromTimer::<T>::from_partial(
            timer_entity,
            partial_timer_event,
        ));
        if let Some(done_event) = partial_timer_event.try_get_done_event() {
            if let EventFromTimerType::DespawnSelf = done_event {
                commands.entity(timer_entity).despawn();
            } else {
                commands.entity(timer_entity).remove::<CustomTimer<Vec3>>();
            }
        }
    }
}
