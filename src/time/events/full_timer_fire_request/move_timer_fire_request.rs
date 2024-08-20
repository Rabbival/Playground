use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct MoveTimerFireRequest(CalculatingTimer<Vec3>);

impl MoveTimerFireRequest {
    pub fn new(
        movement_type: MovementType,
        calculator: ValueByInterpolation<Vec3>,
        affected_entities: Vec<Entity>,
        time_multipliers: Vec<TimeMultiplierId>,
        duration: f32,
        once_done: TimerDoneEventType,
    ) -> Self {
        Self(CalculatingTimer {
            timer: FullTimer::new(
                affected_entities,
                time_multipliers,
                duration,
                TimerGoingEventType::Move(movement_type),
                once_done,
            ),
            calculator,
        })
    }
}

impl FullTimerFireRequestType for MoveTimerFireRequest {
    fn spawn_timer(&self, commands: &mut Commands) -> Entity {
        commands.spawn(self.0).id()
    }

    fn entities(&self) -> VecBasedArray<Entity, TIMER_MAX_ASSIGNED_ENTITIES> {
        self.0.timer.affected_entities
    }

    fn timer_going_event_type(&self) -> TimerGoingEventType {
        self.0.timer.send_as_going
    }
}
