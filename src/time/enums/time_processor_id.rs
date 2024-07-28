use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeProcessorId {
    GameTimeProcessor,
    UiTimeProcessor,
}
