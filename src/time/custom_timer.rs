use crate::prelude::*;

#[derive(Debug, Component, Default)]
pub struct CustomTimer {
    send_as_going: Option<TimerEvent>,
    send_once_done: Option<TimerEvent>,
    full_duration: f32,
    tick_duration: f32,
    elapsed_ticks: u128,
    elapsed_since_last_tick: f32,
    tick_duration_change_ignorant: bool,
}

impl CustomTimer {
    pub fn new(full_duration: f32, tick_duration: f32) -> Self {
        let clamped_full_duration = full_duration.clamp(0.0, AN_HOUR_IN_SECONDS as f32);
        let clamped_tick_duration = tick_duration.clamp(0.0, MILLIS_IN_SECONDS);
        Self {
            full_duration: clamped_full_duration,
            tick_duration: clamped_tick_duration,
            ..Default::default()
        }
    }

    pub fn set_send_as_going(&mut self, event: TimerEvent) {
        self.send_as_going = Some(event);
    }

    pub fn set_send_once_done(&mut self, event: TimerEvent) {
        self.send_once_done = Some(event);
    }

    pub fn set_tick_duration(&mut self, tick_duration: f32) {
        if !self.tick_duration_change_ignorant {
            self.tick_duration = tick_duration;
        }
    }

    pub fn percentage_to_done(&self) -> f32 {
        (self.elapsed_ticks as f32 * self.tick_duration / self.full_duration) * 100.0
    }

    pub fn is_done(&self) -> bool {
        self.elapsed_ticks as f32 * self.tick_duration >= self.full_duration
    }

    pub fn tick(&mut self, delta: f32) -> (Option<TimerEvent>, TicksSinceLastUpdate) {
        let optional_timer_event_to_send;
        let ticks_since_last_update;
        self.elapsed_since_last_tick += delta;
        ticks_since_last_update = self.update_timer_ticks();
        optional_timer_event_to_send = self.get_event_to_send(ticks_since_last_update);
        (optional_timer_event_to_send, ticks_since_last_update)
    }

    fn update_timer_ticks(&mut self) -> TicksSinceLastUpdate {
        let mut ticks_since_last_update = TicksSinceLastUpdate::default();
        while self.elapsed_since_last_tick >= self.tick_duration {
            self.elapsed_since_last_tick -= self.tick_duration;
            ticks_since_last_update.0 += 1;
        }
        self.elapsed_ticks += ticks_since_last_update.0 as u128;
        ticks_since_last_update
    }

    fn get_event_to_send(
        &self,
        ticks_since_last_update: TicksSinceLastUpdate,
    ) -> Option<TimerEvent> {
        if ticks_since_last_update.0 > 0 {
            if self.is_done() {
                self.send_once_done.map(|mut event| {
                    event.set_elapsed_percentage(100.0);
                    event
                })
            } else {
                self.send_as_going.map(|mut event| {
                    event.set_elapsed_percentage(self.percentage_to_done());
                    event
                })
            }
        } else {
            None
        }
    }
}
