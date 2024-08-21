use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use crate::{prelude::*, trait_union};

trait_union!(
    Numeric,
    Add<Output = Self> + Sub<Output = Self> + Mul<f32, Output = Self> + Sendable + Default
);

trait_union!(
    SendableTimerFireRequestType,
    FullTimerFireRequestType + Sendable
);

trait_union!(Sendable, Clone + Copy + Send + Sync + 'static + Debug);
