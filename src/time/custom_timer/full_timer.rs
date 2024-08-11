use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub struct FullTimer {
    pub entities: VecBasedArray<Entity, TIMER_MAX_ASSIGNED_ENTITIES>,
    pub time_multipliers: VecBasedArray<TimeMultiplierId, TIMER_MAX_ASSIGNED_MULTIPLIERS>,
    pub send_as_going: TimerGoingEventType,
    pub send_once_done: TimerDoneEventType,
    duration: f32,
    elapsed_time: f32,
    normalized_progress: f32,
}

impl FullTimer {
    pub fn new(
        entities_vec: Vec<Entity>,
        time_multipliers_vec: Vec<TimeMultiplierId>,
        duration: f32,
        send_as_going: TimerGoingEventType,
        send_once_done: TimerDoneEventType,
    ) -> Self {
        let clamped_duration =
            clamp_and_notify(duration, A_MILLISECOND_IN_SECONDS, AN_HOUR_IN_SECONDS);
        let entities_array = VecBasedArray::new(entities_vec);
        let time_multipliers_array = VecBasedArray::new(time_multipliers_vec);
        Self {
            entities: entities_array,
            time_multipliers: time_multipliers_array,
            send_as_going,
            send_once_done,
            duration: clamped_duration,
            elapsed_time: 0.0,
            normalized_progress: 0.0,
        }
    }

    pub fn finished(&self) -> bool {
        self.normalized_progress >= 1.0
    }

    pub fn tick_and_get_normalized_progress(&mut self, processed_time: f32) -> Option<f32> {
        if processed_time > 0.0 && !self.finished() {
            self.elapsed_time += processed_time;
            self.update_normalized_progress();
            Some(self.normalized_progress)
        } else {
            None
        }
    }

    fn update_normalized_progress(&mut self) {
        self.normalized_progress = (self.elapsed_time / self.duration).min(1.0);
    }
}
