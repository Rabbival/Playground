use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum EventFromTimerType {
    #[default]
    Nothing,
    DespawnSelf,
    ChangeTimeMultiplierSpeed,
    MoveInDirectLine,
    RotateAround {
        center: Vec3,
        radius: f32,
    },
}
