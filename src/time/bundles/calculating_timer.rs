use crate::prelude::*;

#[derive(Bundle, Debug, Clone, Copy)]
pub struct CalculatingTimer<T: Numeric> {
    pub timer: FullTimer,
    pub calculator: ValueByInterpolation<T>,
}
