use crate::prelude::*;

pub mod affecting_timer_set_policy;
pub mod affecting_timers_plugin;
pub mod full_timer_affected_entity;

#[derive(Debug, Component, Clone, Default)]
pub struct AffectingTimers(HashMap<TimerGoingEventType, Entity>);

impl AffectingTimers {
    pub fn get(&self, going_event_type: &TimerGoingEventType) -> Option<&Entity> {
        self.0.get(going_event_type)
    }

    pub fn insert(
        &mut self,
        key: TimerGoingEventType,
        value: Entity,
        policy: AffectingTimerSetPolicy,
    ) -> Option<Entity> {
        match policy {
            AffectingTimerSetPolicy::AlwaysTakeNew => self.0.insert(key, value),
            AffectingTimerSetPolicy::IgnoreNewIfAssigned => {
                let maybe_existing_entity = self.get(&key).copied();
                if maybe_existing_entity.is_none() {
                    self.0.insert(key, value);
                }
                maybe_existing_entity
            }
        }
    }

    pub fn remove(&mut self, key: &TimerGoingEventType) -> Option<Entity> {
        self.0.remove(key)
    }
}
