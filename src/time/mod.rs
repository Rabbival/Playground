pub mod bundles;
pub mod consts;
pub mod events;
pub mod time_multiplication;
pub mod time_related_error;
pub mod timer;
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
