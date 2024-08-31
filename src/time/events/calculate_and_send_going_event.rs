use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct CalculateAndSendGoingEvent {
    pub affected_entities: VecBasedArray<FullTimerAffectedEntity, TIMER_MAX_ASSIGNED_ENTITIES>,
    pub normalized_progress: f32,
    pub event_type_to_send: TimerGoingEventType,
}

pub struct CalculateAndSendGoingEventPlugin;

impl Plugin for CalculateAndSendGoingEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CalculateAndSendGoingEvent>();
    }
}
