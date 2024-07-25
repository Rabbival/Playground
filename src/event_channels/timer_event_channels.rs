use crate::prelude::*;

pub trait StoreElapsedPercentage {
    fn set_elapsed_percentage(&mut self, elapsed_percentage: f32);
}

#[derive(Debug, Event, Clone, Copy)]
pub enum TimerEvent {
    AnimationTimerEvent(f32),
    TickChangeEvent(f32),
}

impl StoreElapsedPercentage for TimerEvent {
    fn set_elapsed_percentage(&mut self, elapsed_percentage: f32) {
        match self {
            Self::AnimationTimerEvent(percentage) => *percentage = elapsed_percentage,
            Self::TickChangeEvent(percentage) => *percentage = elapsed_percentage,
        }
    }
}

pub struct TimerEventChannelsPlugin;

impl Plugin for TimerEventChannelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerEvent>();
    }
}
