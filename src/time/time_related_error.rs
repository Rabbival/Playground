use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum TimeRelatedError {
    TimeMultiplierNotFound(TimeMultiplierId),
    AttemptedToChangeFixedMultiplierTimeMultiplier(TimeMultiplierId),
}

impl Display for TimeRelatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::TimeMultiplierNotFound(id) => {
                write!(f, "Time processor with id {:?} not found", id)
            }
            Self::AttemptedToChangeFixedMultiplierTimeMultiplier(id) => write!(
                f,
                "Attempted to change fixed multiplier time processor with id {:?}",
                id
            ),
        }
    }
}
