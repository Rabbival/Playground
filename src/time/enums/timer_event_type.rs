use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum TimerEventType {
    ChangeTimeProcessorSpeed(TimeProcessorId),
    Dummy,
}
