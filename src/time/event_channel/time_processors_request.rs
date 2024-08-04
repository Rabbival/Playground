use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum TimeMultipliersRequest {
    SetTimeMultiplier {
        processor_id: TimeMultiplierId,
        new_multiplier: f32,
        duration: f32,
    },
    AddProcessor(TimeMultiplier),
}
