use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FullTimerAffectedEntity {
    pub affected_entity: Entity,
    pub value_calculator_entity: Entity,
}
