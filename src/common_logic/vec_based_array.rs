use crate::prelude::*;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VecBasedArray<T: Debug + Copy, const N: usize>(pub [Option<T>; N]);

impl<T: Debug + Copy, const N: usize> VecBasedArray<T, N> {
    pub fn new(value: Vec<T>) -> Self {
        let mut array = [None; N];
        let shortened_vec = truncated_if_at_limit(value, N);
        for (i, elem) in shortened_vec.into_iter().enumerate() {
            array[i] = Some(elem);
        }
        Self(array)
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.0.iter().flatten().copied()
    }
}
