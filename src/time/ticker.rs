use crate::prelude::*;

#[derive(Debug, Component, Default, Clone, Copy)]
pub struct Ticker {
    tick_duration: f32,
    elapsed_since_last_tick: f32,
    changable_tick_duration: bool,
}

impl Ticker {
    pub fn new(tick_duration: f32, changable_tick_duration: bool) -> Self {
        let clamped_tick_duration =
            clamp_and_notify(tick_duration, A_MILLISECOND_IN_SECONDS, A_MINUTE_IN_SECONDS);
        Self {
            tick_duration: clamped_tick_duration,
            changable_tick_duration,
            ..Default::default()
        }
    }

    pub fn get_tick_duration(&self) -> f32 {
        self.tick_duration
    }

    pub fn set_tick_duration(&mut self, tick_duration: f32) {
        if self.changable_tick_duration {
            self.tick_duration = tick_duration;
        } else {
            print_warning(
                "attempted to change the tick duration of a fixed-tick-duration timer",
                vec![LogCategory::RequestNotFulfilled],
            )
        }
    }

    pub fn tick(&mut self, time: &Time) -> TicksSinceLastUpdate {
        let mut ticks_since_last_update = TicksSinceLastUpdate::default();
        self.elapsed_since_last_tick += time.delta_seconds();
        while self.elapsed_since_last_tick >= self.tick_duration {
            self.elapsed_since_last_tick -= self.tick_duration;
            ticks_since_last_update.0 += 1;
        }
        ticks_since_last_update
    }
}
