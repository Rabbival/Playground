use crate::prelude::*;

#[derive(Debug, Component, Clone, Default)]
pub struct FullTimerAffected {
    pub affecting_timers: HashMap<TimerGoingEventType, Entity>,
}
