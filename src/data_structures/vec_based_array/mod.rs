use crate::prelude::*;
use std::fmt::Debug;

pub mod vec_based_array_error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VecBasedArray<T: Debug + Copy + PartialEq, const N: usize> {
    pub array: [Option<T>; N],
    next_uninitialized_index: usize,
}

impl<T: Debug + Copy + PartialEq, const N: usize> VecBasedArray<T, N> {
    pub fn new(value: Vec<T>) -> Self {
        let mut array = [None; N];
        let shortened_vec = truncated_if_at_limit(value, N);
        let next_uninitialized_index = shortened_vec.len();
        for (i, elem) in shortened_vec.into_iter().enumerate() {
            array[i] = Some(elem);
        }
        Self {
            array,
            next_uninitialized_index,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        self.array.iter().flatten().copied()
    }

    pub fn len(&self) -> usize {
        self.next_uninitialized_index
    }

    pub fn remove_by_item(&mut self, item_to_remove: T) -> Result<T, VecBasedArrayError<T, N>> {
        let mut maybe_item_index = None;
        for (index, item) in self.iter().enumerate() {
            if item == item_to_remove {
                maybe_item_index = Some(index);
                break;
            }
        }
        match maybe_item_index {
            Some(index) => self.remove_by_index(index),
            None => Err(VecBasedArrayError::ItemNotFound(item_to_remove, *self)),
        }
    }

    pub fn remove_by_index(&mut self, index: usize) -> Result<T, VecBasedArrayError<T, N>> {
        if index < self.next_uninitialized_index {
            let removed_item = self.array[index];
            self.array[index] = None;
            self.next_uninitialized_index -= 1;
            self.array.swap(index, self.next_uninitialized_index);
            Ok(removed_item.unwrap())
        } else {
            Err(VecBasedArrayError::IndexOutOfRange(index, *self))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_based_vec_creation() {
        let vec = vec![1, 2];

        let bigger_vec_based_array: VecBasedArray<usize, 3> = VecBasedArray::new(vec.clone());
        let smaller_vec_based_array: VecBasedArray<usize, 1> = VecBasedArray::new(vec.clone());

        assert_eq!(bigger_vec_based_array.array, [Some(1), Some(2), None]);
        assert_eq!(bigger_vec_based_array.next_uninitialized_index, 2);
        assert_eq!(smaller_vec_based_array.array, [Some(1)]);
        assert_eq!(smaller_vec_based_array.next_uninitialized_index, 1);
    }

    #[test]
    fn test_array_based_vec_iter() {
        let vec = vec![1, 2];

        let bigger_vec_based_array: VecBasedArray<usize, 3> = VecBasedArray::new(vec.clone());
        let smaller_vec_based_array: VecBasedArray<usize, 1> = VecBasedArray::new(vec.clone());

        assert_eq!(bigger_vec_based_array.iter().count(), 2);
        assert_eq!(smaller_vec_based_array.iter().count(), 1);
        assert_eq!(bigger_vec_based_array.iter().next(), Some(1));
        assert_eq!(smaller_vec_based_array.iter().next(), Some(1));
    }

    #[test]
    fn test_remove_by_item() {
        let vec = vec![1, 2];
        let mut vec_based_array: VecBasedArray<usize, 2> = VecBasedArray::new(vec.clone());
        let original_vec_based_array_copy = vec_based_array.clone();

        let valid_removal_result = vec_based_array.remove_by_item(1);
        let item_not_found_removal_result = vec_based_array.remove_by_item(1);

        assert_eq!(2, original_vec_based_array_copy.next_uninitialized_index);
        assert_eq!([Some(2), None], vec_based_array.array);
        assert_eq!(1, vec_based_array.next_uninitialized_index);
        assert_eq!(Ok(1), valid_removal_result);
        assert_eq!(
            Err(VecBasedArrayError::ItemNotFound(1, vec_based_array)),
            item_not_found_removal_result
        );
    }
}
