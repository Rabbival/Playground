use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub struct CustomTimer<T: Numeric> {
    pub time_processor: TimeProcessorId,
    pub send_as_going: Option<EventFromTimerType>,
    pub send_once_done: Option<EventFromTimerType>,
    duration: f32,
    original_value: T,
    goal_value: T,
    elapsed_time: f32,
    normalized_progress: f32,
}

impl<T: Numeric> CustomTimer<T> {
    fn new(
        time_processor: TimeProcessorId,
        duration: f32,
        original_value: T,
        goal_value: T,
        send_as_going: Option<EventFromTimerType>,
        send_once_done: Option<EventFromTimerType>,
    ) -> Self {
        let clamped_duration =
            clamp_and_notify(duration, A_MILLISECOND_IN_SECONDS, AN_HOUR_IN_SECONDS);
        Self {
            time_processor,
            send_as_going,
            send_once_done,
            duration: clamped_duration,
            original_value,
            goal_value,
            elapsed_time: 0.0,
            normalized_progress: 0.0,
        }
    }

    pub fn finished(&self) -> bool {
        self.normalized_progress >= 1.0
    }

    pub fn update_normalized_progress(&mut self) {
        self.normalized_progress = (self.elapsed_time / self.duration).min(1.0);
    }

    pub fn tick_and_get_event(&mut self, processed_time: f32) -> Option<EventFromTimer<T>> {
        if processed_time > 0.0 && !self.finished() {
            self.elapsed_time += processed_time;
            self.update_normalized_progress();
            Some(self.get_event_to_send())
        } else {
            None
        }
    }

    fn get_event_to_send(&self) -> EventFromTimer<T> {
        EventFromTimer::<T>::new(
            self.get_current_value(),
            self.send_as_going,
            if self.finished() {
                self.send_once_done
            } else {
                None
            },
        )
    }

    fn get_current_value(&self) -> T {
        self.original_value + (self.goal_value - self.original_value) * self.normalized_progress
    }
}
