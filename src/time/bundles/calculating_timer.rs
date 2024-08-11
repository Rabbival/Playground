use crate::prelude::*;

#[derive(Bundle)]
pub struct CalculatingTimer<T: Numeric> {
    pub timer: FullTimer,
    pub calculator: ValueByInterpolation<T>,
}
