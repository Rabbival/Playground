use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct MoveTimerFireRequest(FullTimer);

impl MoveTimerFireRequest {
    pub fn new(
        movement_type: MovementType,
        affected_entities: Vec<FullTimerAffectedEntity>,
        time_multipliers: Vec<TimeMultiplierId>,
        duration: f32,
        once_done: TimerDoneEventType,
    ) -> Self {
        Self(FullTimer::new(
            affected_entities,
            time_multipliers,
            duration,
            TimerGoingEventType::Move(movement_type),
            once_done,
        ))
    }
}

impl FullTimerFireRequestType for MoveTimerFireRequest {
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
