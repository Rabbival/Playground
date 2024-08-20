use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct TimeMultiplierChangeTimerFireRequest(CalculatingTimer<f32>);

impl TimeMultiplierChangeTimerFireRequest {
    pub fn new(
        calculator: ValueByInterpolation<f32>,
        multiplier_entity: Entity,
        duration: f32,
    ) -> Self {
        Self(CalculatingTimer {
            timer: FullTimer::new(
                vec![multiplier_entity],
                vec![],
                duration,
                TimerGoingEventType::ChangeTimeMultiplierSpeed,
                TimerDoneEventType::default(),
            ),
            calculator,
        })
    }
}

impl FullTimerFireRequestType for TimeMultiplierChangeTimerFireRequest {
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
