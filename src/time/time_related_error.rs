use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum NonGenericTimeRelatedError {
    TimeProcessorNotFound(TimeProcessorId),
    AttemptedToChangeFixedMultiplierTimeProcessor(TimeProcessorId),
}

pub enum GenericTimeRelatedError<T: Numeric> {
    NoTimeProcessorAssignedToTimer(CustomTimer<T>),
}

impl Display for NonGenericTimeRelatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::TimeProcessorNotFound(id) => {
                write!(f, "Time processor with id {:?} not found", id)
            }
            Self::AttemptedToChangeFixedMultiplierTimeProcessor(id) => write!(
                f,
                "Attempted to change fixed multiplier time processor with id {:?}",
                id
            ),
        }
    }
}

impl<T: Numeric> Display for GenericTimeRelatedError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoTimeProcessorAssignedToTimer(timer) => {
                write!(f, "No time processor assigned to timer {:?}", timer)
            }
        }
    }
}
