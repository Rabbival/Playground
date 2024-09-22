pub mod consts;
pub mod emitting_timer;
pub mod errors;
pub mod events;
pub mod going_event_management;
pub mod time_multiplication;
pub mod timer_affected_entity;
pub mod timer_calculators;
pub mod timer_management;

use crate::prelude::*;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TimerManagementPlugin,
            TimeMutiplicationPlugin,
            TimeEventChannelPlugin,
        ));
    }
}
