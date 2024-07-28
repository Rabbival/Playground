use crate::prelude::*;

#[derive(Debug, Component, Default)]
pub struct TickerTimer {
    pub ticker: Ticker,
    pub send_as_going: Option<TimerEvent>,
    pub send_once_done: Option<TimerEvent>,
    full_duration: f32,
    elapsed_ticks: u128,
    finished: bool,
}

impl TickerTimer {
    pub fn new(ticker: Ticker, full_duration: f32) -> Self {
        let clamped_full_duration =
            clamp_and_notify(full_duration, A_MILLISECOND_IN_SECONDS, AN_HOUR_IN_SECONDS);
        Self {
            full_duration: clamped_full_duration,
            ticker,
            ..Default::default()
        }
    }

    pub fn normalized_progress(&mut self) -> f32 {
        let normalized_progress =
            self.elapsed_ticks as f32 * self.ticker.get_tick_duration() / self.full_duration;
        if normalized_progress >= 1.0 {
            self.finished = true;
            1.0
        } else {
            normalized_progress
        }
    }

    pub fn apply_ticks(
        &mut self,
        ticks_since_last_update: TicksSinceLastUpdate,
    ) -> Option<TimerEvent> {
        let ticks_to_add = ticks_since_last_update.0;
        if ticks_to_add > 0 && !self.finished {
            self.elapsed_ticks += ticks_to_add as u128;
            self.get_event_to_send()
        } else {
            None
        }
    }

    fn get_event_to_send(&mut self) -> Option<TimerEvent> {
        let normalized_progress = self.normalized_progress();
        let event_to_emit = if self.finished {
            self.send_once_done
        } else {
            self.send_as_going
        };
        event_to_emit.map(|mut event| {
            event.set_elapsed_normalized(normalized_progress);
            event
        })
    }
}
