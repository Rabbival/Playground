use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct TimerEvent<T: Numeric> {
    pub value: T,
    pub event_type: TimerEventType,
}

#[derive(Debug, Clone, Copy)]
pub enum TimerEventType {
    ChangeTimeProcessorSpeed(TimeProcessorId),
    Dummy,
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
