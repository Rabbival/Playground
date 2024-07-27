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
            vec![LogCategory::ValueChanges],
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
            vec![LogCategory::ValueChanges],
        );
        max
    } else {
        value
    }
}
