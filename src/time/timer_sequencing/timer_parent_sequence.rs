use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimerParentSequence {
    pub parent_sequence: Entity,
    pub index_in_sequence: usize,
}
