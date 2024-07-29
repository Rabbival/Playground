use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub struct CustomTimer<T: Numeric> {
    pub time_processor: TimeProcessorId,
    pub send_as_going: Option<TimerEvent<T>>,
    pub send_once_done: Option<TimerEvent<T>>,
    original_value: Option<T>,
    goal_value: Option<T>,
    duration: f32,
    elapsed_time: f32,
    finished: bool,
}

impl<T: Numeric> CustomTimer<T> {
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn normalized_progress(&mut self) -> f32 {
        let normalized_progress = self.elapsed_time / self.duration;
        if normalized_progress >= 1.0 {
            self.finished = true;
            1.0
        } else {
            normalized_progress
        }
    }

    pub fn tick_and_get_event(&mut self, processed_time: f32) -> Option<TimerEvent<T>> {
        if processed_time > 0.0 && !self.finished {
            self.elapsed_time += processed_time;
            self.get_event_to_send()
        } else {
            None
        }
    }

    fn get_event_to_send(&mut self) -> Option<TimerEvent<T>> {
        let normalized_progress = self.normalized_progress();
        let event_to_emit = if self.finished {
            self.send_once_done
        } else {
            self.send_as_going
        };
        event_to_emit.map(|mut event| {
            if let (Some(original_value), Some(goal_value)) = (self.original_value, self.goal_value)
            {
                event.value = original_value + (goal_value - original_value) * normalized_progress;
            }
            event
        })
    }
}

// constructors
impl<T: Numeric> CustomTimer<T> {
    pub fn full(
        time_processor: TimeProcessorId,
        duration: f32,
        original_value: T,
        goal_value: T,
        send_as_going: TimerEvent<T>,
        send_once_done: TimerEvent<T>,
    ) -> Self {
        Self::new(
            time_processor,
            duration,
            Some(original_value),
            Some(goal_value),
            Some(send_as_going),
            Some(send_once_done),
        )
    }

    pub fn with_as_going_only(
        time_processor: TimeProcessorId,
        duration: f32,
        original_value: T,
        goal_value: T,
        send_as_going: TimerEvent<T>,
    ) -> Self {
        Self::new(
            time_processor,
            duration,
            Some(original_value),
            Some(goal_value),
            Some(send_as_going),
            None,
        )
    }

    pub fn with_once_done_only(
        time_processor: TimeProcessorId,
        duration: f32,
        send_once_done: TimerEvent<T>,
    ) -> Self {
        Self::new(
            time_processor,
            duration,
            None,
            None,
            None,
            Some(send_once_done),
        )
    }

    fn new(
        time_processor: TimeProcessorId,
        duration: f32,
        original_value: Option<T>,
        goal_value: Option<T>,
        send_as_going: Option<TimerEvent<T>>,
        send_once_done: Option<TimerEvent<T>>,
    ) -> Self {
        let clamped_duration =
            clamp_and_notify(duration, A_MILLISECOND_IN_SECONDS, AN_HOUR_IN_SECONDS);
        Self {
            time_processor,
            send_as_going,
            send_once_done,
            original_value,
            goal_value,
            duration: clamped_duration,
            elapsed_time: 0.0,
            finished: false,
        }
    }
}
