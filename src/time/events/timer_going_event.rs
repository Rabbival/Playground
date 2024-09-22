use crate::{plugin_for_implementors_of_trait, prelude::*};

#[derive(Debug, Clone, Copy, PartialEq, Event)]
pub struct TimerGoingEvent<T: Numeric> {
    pub event_type: TimerGoingEventType,
    pub entity: Entity,
    pub value_delta: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimerGoingEventType {
    ChangeTimeMultiplierSpeed,
    Move(MovementType),
}

plugin_for_implementors_of_trait!(TimerGoingEventPlugin, Numeric);

impl<T: Numeric> Plugin for TimerGoingEventPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerGoingEvent<T>>();
    }
}
