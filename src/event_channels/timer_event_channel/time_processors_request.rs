use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum TimeProcessorsRequest {
    SetTimeMultiplier {
        processor_id: TimeProcessorId,
        new_multiplier: f32,
        duration: f32,
    },
    AddProcessor(TimeProcessor),
}
