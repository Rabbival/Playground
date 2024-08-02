use crate::prelude::*;

pub mod game_event_channels;
pub mod timer_event_channel;
pub mod transform_event_channels;

pub struct EventChannelPlugin;

impl Plugin for EventChannelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameEventChannelsPlugin, TimerEventChannelPlugin, TransformEventChannelsPlugin));
    }
}
