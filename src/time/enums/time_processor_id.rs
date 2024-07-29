use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TimeProcessorId {
    #[default]
    RealTime,
    GameTimeProcessor,
    UiTimeProcessor,
}
