use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub struct MismatchError {
    pub expected: String,
    pub found: String,
}

impl Display for MismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "expected {} but found {}", self.expected, self.found)
    }
}

impl From<MismatchError> for SystemAccessError {
    fn from(mismatch_error: MismatchError) -> Self {
        SystemAccessError::MismatchingPostfix(mismatch_error)
    }
}
