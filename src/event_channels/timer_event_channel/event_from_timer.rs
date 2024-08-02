use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct EventFromTimer<T: Numeric> {
    value: T,
    timer_going: Option<EventFromTimerType>,
    timer_done: Option<EventFromTimerType>,
    relevant_entity: Option<Entity>,
}

impl<T: Numeric> EventFromTimer<T> {
    pub fn new(
        original_value: T,
        send_as_going: Option<EventFromTimerType>,
        send_once_done: Option<EventFromTimerType>,
        relevant_entity: Option<Entity>,
    ) -> Self {
        Self {
            value: original_value,
            timer_going: send_as_going,
            timer_done: send_once_done,
            relevant_entity
        }
    }

    pub fn current_value(&self) -> T {
        self.value
    }

    pub fn try_get_as_going_event(&self) -> Option<EventFromTimerType> {
        self.timer_going
    }

    pub fn try_get_done_event(&self) -> Option<EventFromTimerType> {
        self.timer_done
    }

    pub fn try_get_relevant_entity(&self) -> Option<Entity> {
        self.relevant_entity
    }
}
