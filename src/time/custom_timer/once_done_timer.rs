use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub struct OnceDoneTimer {
    pub affected_entities: VecBasedArray<Entity, TIMER_MAX_ASSIGNED_ENTITIES>,
    pub time_multipliers: VecBasedArray<TimeMultiplierId, TIMER_MAX_ASSIGNED_MULTIPLIERS>,
    pub send_once_done: TimerDoneEventType,
    duration: f32,
    elapsed_time: f32,
}

impl OnceDoneTimer {
    pub fn new(
        affected_entities_vec: Vec<Entity>,
        time_multipliers_vec: Vec<TimeMultiplierId>,
        duration: f32,
        send_once_done: TimerDoneEventType,
    ) -> Self {
        let clamped_duration =
            clamp_and_notify(duration, A_MILLISECOND_IN_SECONDS, AN_HOUR_IN_SECONDS);
        let affected_entities_array = VecBasedArray::new(affected_entities_vec);
        let time_multipliers_array = VecBasedArray::new(time_multipliers_vec);
        Self {
            affected_entities: affected_entities_array,
            time_multipliers: time_multipliers_array,
            send_once_done,
            duration: clamped_duration,
            elapsed_time: 0.0,
        }
    }

    pub fn finished(&self) -> bool {
        self.elapsed_time >= self.duration
    }

    pub fn tick_and_get_event_if_finished(
        &mut self,
        processed_time: f32,
    ) -> Option<TimerDoneEventType> {
        if processed_time > 0.0 && !self.finished() {
            self.elapsed_time += processed_time;
            if self.finished() {
                return Some(self.send_once_done);
            }
        }
        None
    }
}
