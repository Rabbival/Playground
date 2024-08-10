use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum EventFromTimerType {
    #[default]
    Nothing,
    DespawnSelf,
    ChangeTimeMultiplierSpeed,
    Move(MoveEventFromTimer),
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum MoveEventFromTimer {
    #[default]
    InDirectLine,
    InCircleAround {
        center: Vec3,
        radius: f32,
    },
}
