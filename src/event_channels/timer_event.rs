use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct TimerEvent<T: Numeric> {
    pub progress_normalized: f32,
    original_value: T,
    goal_value: T,
    pub event_type: TimerEventType,
}

#[derive(Debug, Clone, Copy)]
pub enum TimerEventType {
    ChangeTimeProcessorSpeed(TimeProcessorId),
    Dummy,
}

impl<T: Numeric> TimerEvent<T> {
    pub fn is_finished(&self) -> bool {
        self.progress_normalized >= 1.0
    }
    pub fn current_value(&self) -> T {
        self.original_value + self.goal_value - self.original_value * self.progress_normalized
    }
}

pub struct TimerEventPlugin;

impl Plugin for TimerEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerEvent<f32>>()
            .add_event::<TimerEvent<Vec2>>()
            .add_event::<TimerEvent<Vec3>>()
            .add_event::<TimerEvent<Quat>>();
    }
}
