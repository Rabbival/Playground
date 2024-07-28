use crate::prelude::*;

#[derive(Debug, Resource, Default)]
pub struct TimeProcessors(pub HashMap<TimeProcessorId, TimeProcessor>);

pub struct TimeProcessorsPlugin;

impl Plugin for TimeProcessorsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimeProcessors>();
    }
}
