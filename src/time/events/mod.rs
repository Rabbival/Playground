use crate::prelude::*;

pub mod add_timer_to_entity;
pub mod event_from_timer;
pub mod event_from_timer_type;
pub mod set_time_multiplier;

pub struct TimeEventChannelPlugin;

impl Plugin for TimeEventChannelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EventFromTimerPlugin,
            AddTimerToEntityPlugin,
            SetTimeMultiplierPlugin,
        ));
    }
}
