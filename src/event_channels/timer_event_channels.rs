use crate::prelude::*;

pub trait StoreElapsedNormalized {
    fn set_elapsed_normalized(&mut self, elapsed_normalized: f32);
}

#[derive(Debug, Event, Clone, Copy)]
pub enum TimerEvent {
    AnimationTimerEvent(f32),
    TickChangeEvent(f32),
}

impl StoreElapsedNormalized for TimerEvent {
    fn set_elapsed_normalized(&mut self, elapsed_normalized: f32) {
        match self {
            Self::AnimationTimerEvent(percentage) => *percentage = elapsed_normalized,
            Self::TickChangeEvent(percentage) => *percentage = elapsed_normalized,
        }
    }
}

pub struct TimerEventChannelsPlugin;

impl Plugin for TimerEventChannelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerEvent>();
    }
}
