use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct TimerDoneEvent {
    pub event_type: TimerDoneEventType,
    pub affected_entities: VecBasedArray<TimerAffectedEntity, TIMER_MAX_ASSIGNED_ENTITIES>,
    pub timer_entity: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum TimerDoneEventType {
    #[default]
    Nothing,
    DespawnAffectedEntities,
}

pub struct TimerDoneEventPlugin;

impl Plugin for TimerDoneEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerDoneEvent>();
    }
}
