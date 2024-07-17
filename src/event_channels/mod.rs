use crate::prelude::*;

pub mod game_event_channels;

pub struct EventChannelPlugin;

impl Plugin for EventChannelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameEventChannelsPlugin);
    }
}
