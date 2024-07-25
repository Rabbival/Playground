use crate::prelude::*;

#[derive(Debug, Event)]
pub enum TimerEvent {
    AnimationTimerEvent,
    Temp,
}

pub struct TimerEventChannelsPlugin;

impl Plugin for TimerEventChannelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerEvent>();
    }
}
