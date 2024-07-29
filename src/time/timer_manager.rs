use crate::prelude::*;
use std::ops::{Add, Mul, Sub};

pub struct TimerManagerPlugin;

impl Plugin for TimerManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    tick_timers::<f32>,
                    tick_timers::<Vec2>,
                    tick_timers::<Vec3>,
                    tick_timers::<Quat>,
                ),
                listen_for_time_multiplier_requests,
            )
                .chain(),
        );
    }
}

fn tick_timers<
    T: Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Copy + Send + Sync + 'static,
>(
    mut timer_event_writer: EventWriter<TimerEvent<T>>,
    mut timers: Query<(&mut CustomTimer<T>, Entity)>,
    time_processors: Res<TimeProcessors>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let time_delta = time.delta_seconds();
    for (mut timer, timer_entity) in &mut timers {
        let time_multiplier = get_time_multiplier(&time_processors, &timer);
        tick_and_send_timer_event(
            time_delta * time_multiplier,
            &mut timer,
            timer_entity,
            &mut timer_event_writer,
            &mut commands,
        );
    }
}

fn get_time_multiplier<
    T: Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Copy + Send + Sync + 'static,
>(
    time_processors: &Res<TimeProcessors>,
    timer: &CustomTimer<T>,
) -> f32 {
    if let Some(timer_processor_id) = timer.time_processor {
        for (processor_id, time_processor) in time_processors.0.iter() {
            if timer_processor_id == *processor_id {
                return time_processor.get_time_multiplier();
            }
        }
    }
    1.0
}

fn tick_and_send_timer_event<
    T: Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Copy + Send + Sync + 'static,
>(
    time_to_tick: f32,
    timer: &mut CustomTimer<T>,
    timer_entity: Entity,
    timer_event_writer: &mut EventWriter<TimerEvent<T>>,
    commands: &mut Commands,
) {
    if let Some(timer_event) = timer.tick_and_get_event(time_to_tick) {
        timer_event_writer.send(timer_event);
    }
    if timer.is_finished() {
        commands.entity(timer_entity).despawn();
    }
}

fn listen_for_time_multiplier_requests(mut time_processors: ResMut<TimeProcessors>) {}
