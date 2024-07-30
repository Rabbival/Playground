pub mod consts;
pub mod custom_timer;
pub mod enums;
pub mod time_processor;
pub mod time_processors;
pub mod time_related_error;
pub mod timer_manager;

use crate::prelude::*;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TimerManagerPlugin, TimeProcessorsPlugin));
    }
}
