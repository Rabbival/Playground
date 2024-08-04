use crate::prelude::*;

pub mod event_from_timer;
pub mod event_from_timer_type;
pub mod time_processors_request;

#[derive(Debug, Event, Clone, Copy)]
pub enum TimerEventChannel<T: Numeric> {
    EventFromTimer(EventFromTimer<T>),
    ProcessorsRequest(TimeMultipliersRequest),
}

pub struct TimerEventChannelPlugin;

impl Plugin for TimerEventChannelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerEventChannel<f32>>()
            .add_event::<TimerEventChannel<Vec2>>()
            .add_event::<TimerEventChannel<Vec3>>()
            .add_event::<TimerEventChannel<Quat>>();
    }
}
