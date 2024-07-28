use crate::prelude::*;

pub struct TimerManagerPlugin;

impl Plugin for TimerManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (tick_timers, listen_for_time_multiplier_requests).chain(),
        );
    }
}

fn tick_timers(
    mut timers: Query<&mut CustomTimer>,
    time_processors: Res<TimeProcessors>,
    time: Res<Time>,
) {
    let time_delta = time.delta_seconds();
    for mut timer in &mut timers {
        let mut time_multiplier = 1.0;
        if let Some(timer_processor_id) = timer.time_processor {
            for (processor_id, time_processor) in time_processors.0.iter() {
                if timer_processor_id == *processor_id {
                    time_multiplier = time_processor.get_time_multiplier();
                    break;
                }
            }
        }
        timer.tick(time_delta * time_multiplier);
    }
}

fn listen_for_time_multiplier_requests(mut time_processors: ResMut<TimeProcessors>) {}
