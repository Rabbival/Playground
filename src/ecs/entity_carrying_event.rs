use crate::prelude::*;

pub trait EntityCarryingEvent {
    fn event_entity(&self) -> Entity;
}
