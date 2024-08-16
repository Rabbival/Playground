use std::fmt::{Debug, Display};

use crate::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VecBasedArrayError<T: Debug + Copy + PartialEq, const N: usize> {
    ItemNotFound(T, VecBasedArray<T, N>),
    IndexOutOfRange(usize, VecBasedArray<T, N>),
}

impl<T: Debug + Copy + PartialEq, const N: usize> Display for VecBasedArrayError<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ItemNotFound(item, vec_based_array) => {
                write!(
                    f,
                    "Couldn't find item: {:?} in vec-based array: {:?} ",
                    item, vec_based_array
                )
            }
            Self::IndexOutOfRange(index, vec_based_array) => {
                write!(
                    f,
                    "Index: {:?} out of range for vec-based array: {:?} ",
                    index, vec_based_array
                )
            }
        }
    }
}
