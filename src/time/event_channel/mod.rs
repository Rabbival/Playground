use crate::prelude::*;

pub mod event_from_timer;
pub mod event_from_timer_type;

#[derive(Debug, Event, Clone, Copy)]
pub enum TimeEventChannel<T: Numeric> {
    EventFromTimer(Entity, EventFromTimer<T>),
    SetTimeMultiplier {
        id: TimeMultiplierId,
        new_multiplier: f32,
        duration: f32,
    },
    AddTimerToEntity(Entity, CustomTimer<T>),
}

pub struct TimeEventChannelPlugin;

impl Plugin for TimeEventChannelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimeEventChannel<f32>>()
            .add_event::<TimeEventChannel<Vec2>>()
            .add_event::<TimeEventChannel<Vec3>>()
            .add_event::<TimeEventChannel<Quat>>();
    }
}
