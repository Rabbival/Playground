use crate::prelude::*;

pub mod consts;
pub mod tags;
pub mod orb;
pub mod event_channels;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((OrbPlugin, GameEventChannelsPlugin));
    }
}