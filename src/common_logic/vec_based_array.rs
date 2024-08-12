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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_based_vec_creation() {
        let vec = vec![1, 2];
        let bigger_vec_based_array: VecBasedArray<usize, 3> = VecBasedArray::new(vec.clone());
        assert_eq!(bigger_vec_based_array.0, [Some(1), Some(2), None]);
        let smaller_vec_based_array: VecBasedArray<usize, 1> = VecBasedArray::new(vec.clone());
        assert_eq!(smaller_vec_based_array.0, [Some(1)]);
    }

    #[test]
    fn test_array_based_vec_iter() {
        let vec = vec![1, 2];
        let bigger_vec_based_array: VecBasedArray<usize, 3> = VecBasedArray::new(vec.clone());
        assert_eq!(bigger_vec_based_array.iter().count(), 2);
        let smaller_vec_based_array: VecBasedArray<usize, 1> = VecBasedArray::new(vec.clone());
        assert_eq!(smaller_vec_based_array.iter().count(), 1);
        assert_eq!(bigger_vec_based_array.iter().next(), Some(1));
        assert_eq!(smaller_vec_based_array.iter().next(), Some(1));
    }
}
