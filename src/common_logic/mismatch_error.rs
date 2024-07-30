use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MismatchError {
    pub expected: String,
    pub found: String,
}

impl Display for MismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "expected {} but found {}", self.expected, self.found)
    }
}
