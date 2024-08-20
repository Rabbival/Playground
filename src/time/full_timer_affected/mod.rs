use crate::prelude::*;

pub mod affecting_timer_set_policy;
pub mod full_timer_affected_plugin;

#[derive(Debug, Component, Clone, Default)]
pub struct FullTimerAffected {
    affecting_timers: HashMap<TimerGoingEventType, Entity>,
}

impl FullTimerAffected {
    pub fn get(&self, going_event_type: &TimerGoingEventType) -> Option<&Entity> {
        self.affecting_timers.get(going_event_type)
    }

    pub fn insert(
        &mut self,
        key: TimerGoingEventType,
        value: Entity,
        policy: AffectingTimerSetPolicy,
    ) -> Option<Entity> {
        match policy {
            AffectingTimerSetPolicy::AlwaysTakeNew => self.affecting_timers.insert(key, value),
            AffectingTimerSetPolicy::IgnoreNewIfAssigned => {
                let maybe_existing_entity = self.get(&key).copied();
                if maybe_existing_entity.is_none() {
                    self.affecting_timers.insert(key, value);
                }
                maybe_existing_entity
            }
        }
    }

    pub fn remove(&mut self, key: &TimerGoingEventType) -> Option<Entity> {
        self.affecting_timers.remove(key)
    }
}
