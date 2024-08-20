use crate::prelude::*;

pub trait FullTimerFireRequestType {
    fn spawn_timer(&self, commands: &mut Commands) -> Entity;

    fn entities(&self) -> VecBasedArray<Entity, TIMER_MAX_ASSIGNED_ENTITIES>;

    fn timer_going_event_type(&self) -> TimerGoingEventType;
}
