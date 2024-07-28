use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub enum TimerEvent {
    AnimationTimerEvent(f32),
    ChangeTimeMultiplier(f32),
}

pub struct TimerEventChannelsPlugin;

impl Plugin for TimerEventChannelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerEvent>();
    }
}
