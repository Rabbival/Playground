use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventFromTimerType {
    ChangeTimeProcessorSpeed(TimeProcessorId),
    MoveInDirectLine,
    RotateAround{
        center: Vec3,
        radius: f32,
    }
}
