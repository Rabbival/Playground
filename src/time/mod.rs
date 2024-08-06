pub mod consts;
pub mod custom_timer;
pub mod event_channels;
pub mod time_multiplication;
pub mod time_related_error;
pub mod timer_manager;
pub mod timer_value_calculator;

use crate::prelude::*;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TimerManagerPlugin,
            TimeMutiplicationPlugin,
            TimeEventChannelPlugin,
        ));
    }
}
