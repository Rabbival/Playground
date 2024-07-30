use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum EventFromTimerType {
    ChangeTimeProcessorSpeed(TimeProcessorId),
    Dummy,
}
