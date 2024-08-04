use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub struct CustomTimer<T: Numeric> {
    pub time_multiplier: TimeMultiplierId,
    pub send_as_going: Option<EventFromTimerType>,
    pub send_once_done: EventFromTimerType,
    value_calculator: TimerValueCalculator<T>,
    duration: f32,
    elapsed_time: f32,
    normalized_progress: f32,
}

impl<T: Numeric> CustomTimer<T> {
    pub fn new(
        time_multiplier: TimeMultiplierId,
        duration: f32,
        value_calculator: TimerValueCalculator<T>,
        send_as_going: Option<EventFromTimerType>,
        send_once_done: Option<EventFromTimerType>,
    ) -> Self {
        let clamped_duration =
            clamp_and_notify(duration, A_MILLISECOND_IN_SECONDS, AN_HOUR_IN_SECONDS);
        let send_once_done = send_once_done.unwrap_or_default();
        Self {
            time_multiplier,
            send_as_going,
            send_once_done,
            duration: clamped_duration,
            value_calculator,
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
            self.value_calculator
                .calculate_current_value(self.normalized_progress),
            self.send_as_going,
            if self.finished() {
                Some(self.send_once_done)
            } else {
                None
            },
        )
    }
}
