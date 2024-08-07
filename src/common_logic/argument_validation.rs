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
            vec![
                LogCategory::ValueValidation,
                LogCategory::RequestNotFulfilled,
            ],
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
            vec![
                LogCategory::ValueValidation,
                LogCategory::RequestNotFulfilled,
            ],
        );
        max
    } else {
        value
    }
}

pub fn array_from_vec<T: Debug + Copy, const N: usize>(vec: Vec<T>) -> [Option<T>; N] {
    let mut array = [None; N];
    let shortened_vec = truncated_if_at_limit(vec, N);
    for (i, elem) in shortened_vec.into_iter().enumerate() {
        array[i] = Some(elem);
    }
    array
}

pub fn truncated_if_at_limit<T: Debug>(vec: Vec<T>, max_count: usize) -> Vec<T> {
    if vec.len() > max_count {
        print_warning(
            format!(
                "{:?} reached max count {}, shortning to max",
                vec, max_count
            ),
            vec![
                LogCategory::ValueValidation,
                LogCategory::RequestNotFulfilled,
            ],
        );
        vec.into_iter().take(max_count).collect()
    } else {
        vec
    }
}
