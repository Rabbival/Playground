use strum_macros::EnumIter;

use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash, Default, EnumIter)]
pub enum TimeProcessorId {
    #[default]
    RealTime,
    GameTimeProcessor,
    UiTimeProcessor,
}

impl TimeProcessorId {
    pub fn to_initial_properties(&self) -> TimeProcessor {
        TimeProcessor::new(
            *self, 
            1.0,  
            *self == TimeProcessorId::GameTimeProcessor
        )
    }
}
