use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub enum TranslationEventChannel {
    InitiateMoveInDirectLine {
        entity: Entity,
        origin: Vec3,
        target: Vec3,
        duration: f32,
        once_done: TimerDoneEventType,
    },
}

pub struct AnimationEventChannelsPlugin;

impl Plugin for AnimationEventChannelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TranslationEventChannel>();
    }
}
