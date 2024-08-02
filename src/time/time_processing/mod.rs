use crate::prelude::*;

pub mod time_processor;
pub mod time_processors;
pub mod time_processors_update;
pub mod time_processor_id;

pub struct TimeProcessingPlugin;

impl Plugin for TimeProcessingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TimeProcessorsInitPlugin, TimeProcessorsUpdatePlugin));
    }
}
