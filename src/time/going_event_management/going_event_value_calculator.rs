use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct GoingEventValueCalculator<T: Numeric> {
    pub set_policy: TimerCalculatorSetPolicy,
    calculator: ValueByInterpolation<T>,
    going_event_type: TimerGoingEventType,
}

impl<T: Numeric> GoingEventValueCalculator<T> {
    pub fn new(
        set_policy: TimerCalculatorSetPolicy,
        calculator: ValueByInterpolation<T>,
        going_event_type: TimerGoingEventType,
    ) -> Self {
        Self {
            set_policy,
            calculator,
            going_event_type,
        }
    }

    pub fn get_timer_going_event(
        &self,
        normalized_progress: f32,
        affect_entity: Entity,
    ) -> TimerGoingEvent<T> {
        let current_value = self.calculator.calculate_current_value(normalized_progress);
        TimerGoingEvent {
            event_type: self.going_event_type,
            entity: affect_entity,
            value: current_value,
        }
    }

    pub fn going_event_type(&self) -> TimerGoingEventType {
        self.going_event_type
    }
}
