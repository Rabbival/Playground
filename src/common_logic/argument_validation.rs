use std::any::TypeId;

use crate::prelude::*;

pub fn clamp_and_notify<T: PartialOrd + 'static>(value: T, min: T, max: T) -> T {
    if value < min {
        print_warning(
            format!(
                "value of type {:?} too small, fixed to min.",
                TypeId::of::<T>()
            ),
            vec![LogCategory::ValueChanges],
        );
        min
    } else if value > max {
        print_warning(
            format!(
                "value of type {:?} too big, fixed to max.",
                TypeId::of::<T>()
            ),
            vec![LogCategory::ValueChanges],
        );
        max
    } else {
        value
    }
}
