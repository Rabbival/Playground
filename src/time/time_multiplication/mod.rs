use crate::prelude::*;

pub mod time_multiplier;
pub mod time_multiplier_id;
pub mod time_multiplier_plugin;
pub mod time_multipliers_map;

pub struct TimeMutiplicationPlugin;

impl Plugin for TimeMutiplicationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TimeMultiplierPlugin, TimeMultipliersMapPlugin));
    }
}
