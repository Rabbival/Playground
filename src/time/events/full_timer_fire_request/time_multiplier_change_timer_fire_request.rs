use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct TimeMultiplierChangeTimerFireRequest(FullTimer);

impl TimeMultiplierChangeTimerFireRequest {
    pub fn new(affected_entities: Vec<FullTimerAffectedEntity>, duration: f32) -> Self {
        Self(FullTimer::new(
            affected_entities,
            vec![],
            duration,
            TimerGoingEventType::ChangeTimeMultiplierSpeed,
            TimerDoneEventType::default(),
        ))
    }
}

impl FullTimerFireRequestType for TimeMultiplierChangeTimerFireRequest {
    fn spawn_timer(&self, commands: &mut Commands) -> Entity {
        commands.spawn(self.0).id()
    }

    fn entities(&self) -> VecBasedArray<FullTimerAffectedEntity, TIMER_MAX_ASSIGNED_ENTITIES> {
        self.0.affected_entities
    }

    fn timer_going_event_type(&self) -> TimerGoingEventType {
        self.0.send_as_going
    }
}
