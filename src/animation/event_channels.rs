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

impl TranslationEventChannel {
    pub fn timer_going_event_type(&self) -> TimerGoingEventType {
        match self {
            Self::InitiateMoveInDirectLine { .. } => {
                TimerGoingEventType::Move(MoveEventFromTimer::InDirectLine)
            }
        }
    }
}

impl EntityCarryingEvent for TranslationEventChannel {
    fn event_entity(&self) -> Entity {
        match self {
            Self::InitiateMoveInDirectLine { entity, .. } => *entity,
        }
    }
}
