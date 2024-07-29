use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use crate::trait_union;

trait_union!(
    Numeric,
    Add<Output = Self>
        + Sub<Output = Self>
        + Mul<f32, Output = Self>
        + Copy
        + Send
        + Sync
        + 'static
        + Debug
);
