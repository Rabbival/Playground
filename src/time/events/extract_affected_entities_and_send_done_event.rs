use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct ExtractAffectedEntitiesAndSendDoneEvent {
    pub timer_entity: Entity,
}

pub struct ExtractAffectedEntitiesAndSendDoneEventPlugin;

impl Plugin for ExtractAffectedEntitiesAndSendDoneEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExtractAffectedEntitiesAndSendDoneEvent>();
    }
}
