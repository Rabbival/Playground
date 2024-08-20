use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Event)]
pub struct TimerGoingEvent<T: Numeric> {
    pub event_type: TimerGoingEventType,
    pub entities: VecBasedArray<Entity, TIMER_MAX_ASSIGNED_ENTITIES>,
    pub value: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimerGoingEventType {
    ChangeTimeMultiplierSpeed,
    Move(MovementType),
}

pub struct TimerGoingEventPlugin;

impl Plugin for TimerGoingEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerGoingEvent<f32>>()
            .add_event::<TimerGoingEvent<Vec2>>()
            .add_event::<TimerGoingEvent<Vec3>>()
            .add_event::<TimerGoingEvent<Quat>>();
    }
}
