pub mod consts;
pub mod custom_timer;
pub mod events;
pub mod time_multiplication;
pub mod time_related_error;

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
