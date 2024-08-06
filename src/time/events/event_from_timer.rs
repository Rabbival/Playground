use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct EventFromTimer<T: Numeric> {
    entity: Entity,
    value: T,
    timer_going: Option<EventFromTimerType>,
    timer_done: Option<EventFromTimerType>,
}

#[derive(Debug, Clone, Copy)]
pub struct EventFromTimerNoEntity<T: Numeric> {
    value: T,
    timer_going: Option<EventFromTimerType>,
    timer_done: Option<EventFromTimerType>,
}

pub struct EventFromTimerPlugin;

impl Plugin for EventFromTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventFromTimer<f32>>()
            .add_event::<EventFromTimer<Vec2>>()
            .add_event::<EventFromTimer<Vec3>>()
            .add_event::<EventFromTimer<Quat>>();
    }
}

impl<T: Numeric> EventFromTimer<T> {
    pub fn from_partial(entity: Entity, partial_event: EventFromTimerNoEntity<T>) -> Self {
        Self {
            entity,
            value: partial_event.value,
            timer_going: partial_event.timer_going,
            timer_done: partial_event.timer_done,
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

    pub fn entity(&self) -> Entity {
        self.entity
    }
}

impl<T: Numeric> EventFromTimerNoEntity<T> {
    pub fn new(
        original_value: T,
        send_as_going: Option<EventFromTimerType>,
        send_once_done: Option<EventFromTimerType>,
    ) -> Self {
        Self {
            value: original_value,
            timer_going: send_as_going,
            timer_done: send_once_done,
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
}
