use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Event)]
pub struct TimerDoneEvent {
    pub event_type: TimerDoneEventType,
    pub timer_entity: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TimerDoneEventType {
    #[default]
    Nothing,
    DespawnEntity(Entity),
}

pub struct TimerDoneEventPlugin;

impl Plugin for TimerDoneEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerDoneEvent>();
    }
}
