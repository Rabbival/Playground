use std::ops::{Add, Mul, Sub};

use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy)]
pub struct CustomTimer<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Copy + Send + Sync + 'static,
{
    pub time_processor: Option<TimeProcessorId>,
    pub send_as_going: Option<TimerEvent<T>>,
    pub send_once_done: Option<TimerEvent<T>>,
    duration: f32,
    elapsed_time: f32,
    finished: bool,
}

impl<T> CustomTimer<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<f32, Output = T> + Copy + Send + Sync + 'static,
{
    pub fn new(
        time_processor: Option<TimeProcessorId>,
        duration: f32,
        send_as_going: Option<TimerEvent<T>>,
        send_once_done: Option<TimerEvent<T>>,
    ) -> Self {
        let clamped_duration =
            clamp_and_notify(duration, A_MILLISECOND_IN_SECONDS, AN_HOUR_IN_SECONDS);
        Self {
            time_processor,
            send_as_going,
            send_once_done,
            duration: clamped_duration,
            elapsed_time: 0.0,
            finished: false,
        }
    }

    pub fn from_time_processor_and_duration(
        time_processor: TimeProcessorId,
        duration: f32,
    ) -> Self {
        Self::new(Some(time_processor), duration, None, None)
    }

    pub fn from_duration(duration: f32) -> Self {
        Self::new(None, duration, None, None)
    }

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
            event.progress_normalized = normalized_progress;
            event
        })
    }
}
