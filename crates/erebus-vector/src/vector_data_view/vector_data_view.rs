// === Imports ===

use crate::prelude::*;

// === Types ===

/// A zero-copy borrowed view of [`VectorData<T>`].
/// Holds slices of both data and validity bitmap.
#[derive(Debug, Clone, Copy)]
pub struct VectorDataView<'a, T> {
    pub data: &'a [T],
    pub validity: &'a BitSlice,
}

// === Impl ===

impl<'a, T> VectorDataView<'a, T> {

    /// Number of elements in this view.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns whether the value at index `i` is valid (non-null).
    #[inline]
    pub fn is_valid_at(&self, i: usize) -> bool {
        self.validity.get(i).map(|b| *b).unwrap_or(false)
    }

    /// Returns an element reference if valid.
    #[inline]
    pub fn get(&self, i: usize) -> Option<&'a T> {
        if self.is_valid_at(i) {
            self.data.get(i)
        } else {
            None
        }
    }

    /// Iterates only valid elements.
    #[inline]
    pub fn iter_valid(&self) -> impl Iterator<Item = &'a T> {
        self.data
            .iter()
            .zip(self.validity.iter())
            .filter_map(|(x, v)| if *v { Some(x) } else { None })
    }
}