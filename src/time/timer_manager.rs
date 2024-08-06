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
    mut time_event_writer: EventWriter<TimeEventChannel<T>>,
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
            &mut time_event_writer,
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
    time_event_writer: &mut EventWriter<TimeEventChannel<T>>,
    commands: &mut Commands,
) {
    if let Some(timer_event) = timer.tick_and_get_event(time_to_tick) {
        time_event_writer.send(TimeEventChannel::EventFromTimer(timer_entity, timer_event));
        if let Some(done_event) = timer_event.try_get_done_event() {
            if let EventFromTimerType::DespawnSelf = done_event {
                commands.entity(timer_entity).despawn();
            } else {
                commands.entity(timer_entity).remove::<CustomTimer<Vec3>>();
            }
        }
    }
}
