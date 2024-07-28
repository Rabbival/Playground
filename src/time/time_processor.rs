use crate::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct TimeProcessor {
    time_multiplier: f32,
    changable_time_multiplier: bool,
}

impl TimeProcessor {
    pub fn new(time_multiplier: f32, changable_time_multiplier: bool) -> Self {
        let clamped_time_multiplier =
            clamp_and_notify(time_multiplier, MIN_TIME_MULTIPLIER, MAX_TIME_MULTIPLIER);
        Self {
            time_multiplier: clamped_time_multiplier,
            changable_time_multiplier,
        }
    }

    pub fn get_time_multiplier(&self) -> f32 {
        self.time_multiplier
    }

    pub fn set_multiplier(&mut self, time_multiplier: f32) {
        if self.changable_time_multiplier {
            self.time_multiplier = time_multiplier;
        } else {
            print_warning(
                "attempted to change the multiplier of a fixed-multiplier time processor",
                vec![LogCategory::RequestNotFulfilled],
            )
        }
    }
}
