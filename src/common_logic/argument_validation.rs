use std::{any::TypeId, fmt::Debug};

use crate::prelude::*;

pub fn clamp_and_notify<T: PartialOrd + Debug + 'static>(value: T, min: T, max: T) -> T {
    if value < min {
        print_warning(
            format!(
                "value of type {:?} had value below min: {:?},\n
                fixed to min: {:?}.",
                TypeId::of::<T>(),
                value,
                min
            ),
            vec![LogCategory::ValueValidation],
        );
        min
    } else if value > max {
        print_warning(
            format!(
                "value of type {:?} had value above max: {:?},\n
                fixed to max: {:?}.",
                TypeId::of::<T>(),
                value,
                max
            ),
            vec![LogCategory::ValueValidation],
        );
        max
    } else {
        value
    }
}
