use crate::prelude::*;

#[derive(Debug, Component, Clone, Default)]
pub struct AffectingTimerCalculators(HashMap<TimerGoingEventType, TimerAndCalculator>);

#[derive(Debug, Copy, Clone)]
pub struct TimerAndCalculator {
    pub timer: Entity,
    pub value_calculator: Entity,
}

impl AffectingTimerCalculators {
    pub fn get(&self, going_event_type: &TimerGoingEventType) -> Option<&TimerAndCalculator> {
        self.0.get(going_event_type)
    }

    pub fn insert(
        &mut self,
        key: TimerGoingEventType,
        value: TimerAndCalculator,
        policy: TimerCalculatorSetPolicy,
    ) -> Option<TimerAndCalculator> {
        match policy {
            TimerCalculatorSetPolicy::AlwaysTakeNew => self.0.insert(key, value),
            TimerCalculatorSetPolicy::IgnoreNewIfAssigned => {
                let maybe_existing_entity = self.get(&key).copied();
                if maybe_existing_entity.is_none() {
                    self.0.insert(key, value);
                }
                maybe_existing_entity
            }
        }
    }

    pub fn remove(&mut self, key: &TimerGoingEventType) -> Option<TimerAndCalculator> {
        self.0.remove(key)
    }
}
