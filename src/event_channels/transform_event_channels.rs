use crate::prelude::*;


#[derive(Debug, Event, Clone, Copy)]
pub enum TranslationEventChannel {
    MoveInDirectLine{
        entity: Entity,
        origin: Vec3,
        target: Vec3,
        duration: f32,
        once_done: Option<EventFromTimerType>,
    }
}

pub struct TransformEventChannelsPlugin;

impl Plugin for TransformEventChannelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TranslationEventChannel>();
    }
}