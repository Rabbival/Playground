use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventFromTimerType {
    ChangeTimeProcessorSpeed(TimeProcessorId),
    Dummy,
}
