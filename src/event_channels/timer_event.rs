use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct TimerEvent<T: Numeric> {
    value: T,
    timer_going: Option<TimerEventType>,
    timer_done: Option<TimerEventType>,
}

impl<T: Numeric> TimerEvent<T> {
    pub fn new(
        original_value: T,
        send_as_going: Option<TimerEventType>,
        send_once_done: Option<TimerEventType>,
    ) -> Self {
        Self {
            value: original_value,
            timer_going: send_as_going,
            timer_done: send_once_done,
        }
    }

    pub fn get_current_value(&self) -> T {
        self.value
    }

    pub fn try_get_as_going_event(&self) -> Option<TimerEventType> {
        self.timer_going
    }

    pub fn try_get_done_event(&self) -> Option<TimerEventType> {
        self.timer_done
    }

    pub fn timer_done(&self) -> bool {
        self.timer_done.is_some()
    }
}

pub struct TimerEventPlugin;

impl Plugin for TimerEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerEvent<f32>>()
            .add_event::<TimerEvent<Vec2>>()
            .add_event::<TimerEvent<Vec3>>()
            .add_event::<TimerEvent<Quat>>();
    }
}
