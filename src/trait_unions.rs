use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use crate::{prelude::*, trait_union};

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
        + Default
);

trait_union!(
    SendableTimerFireRequestType,
    FullTimerFireRequestType + Send + Sync + Debug + Clone + Copy + 'static
);
