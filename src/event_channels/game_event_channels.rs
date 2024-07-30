use crate::prelude::*;

#[derive(Debug, Event)]
pub enum GameSpawnRequest {
    SpawnOrb(Vec2),
    PutOtherSpawnRequestsHere,
}

pub struct GameEventChannelsPlugin;

impl Plugin for GameEventChannelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameSpawnRequest>();
    }
}
