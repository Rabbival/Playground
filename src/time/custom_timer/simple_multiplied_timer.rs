use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub struct SimpleMultipliedTimer {
    pub time_multipliers: [Option<TimeMultiplierId>; MAX_ASSIGNED_MULTIPLIERS],
    pub send_once_done: EventFromTimerType,
    duration: f32,
    elapsed_time: f32,
}

impl SimpleMultipliedTimer {
    pub fn new(
        time_multipliers_vec: Vec<TimeMultiplierId>,
        duration: f32,
        send_once_done: EventFromTimerType,
    ) -> Self {
        let clamped_duration =
            clamp_and_notify(duration, A_MILLISECOND_IN_SECONDS, AN_HOUR_IN_SECONDS);
        let time_multipliers_array = array_from_vec(time_multipliers_vec);
        Self {
            time_multipliers: time_multipliers_array,
            send_once_done,
            duration: clamped_duration,
            elapsed_time: 0.0,
        }
    }

    pub fn finished(&self) -> bool {
        self.elapsed_time >= self.duration
    }

    pub fn tick_and_get_event(&mut self, processed_time: f32) -> Option<EventFromTimerType> {
        if processed_time > 0.0 && !self.finished() {
            self.elapsed_time += processed_time;
            if self.finished() {
                return Some(self.send_once_done);
            }
        }
        None
    }
}
