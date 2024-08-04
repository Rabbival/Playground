use crate::prelude::*;

pub mod time_multiplier;
pub mod time_multiplier_id;
pub mod time_processors;
pub mod time_processors_update;

pub struct TimeProcessingPlugin;

impl Plugin for TimeProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TimeMultipliersInitPlugin, TimeMultipliersUpdatePlugin));
    }
}
