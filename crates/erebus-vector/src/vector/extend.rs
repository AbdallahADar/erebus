// === Imports ===
use crate::prelude::*;

// === Impl ===

impl<T: Clone + Send + Sync> Vector<T> {

    /// Extends this `Vector` in place from a raw slice.
    #[inline]
    pub fn extend(&mut self, slice: &[T]) {
        self.data.extend_from_slice(slice);
    }

    /// Appends another `Vector` in place.
    #[inline]
    pub fn append(&mut self, other: &Vector<T>) {
        self.data.extend_from_slice(&other.data);
    }

    /// Returns a new `Vector` by concatenating two others.
    #[inline]
    pub fn concat(&self, other: &Vector<T>) -> Vector<T> {
        let mut data = Vec::with_capacity(self.data.len() + other.data.len());
        data.extend_from_slice(&self.data);
        data.extend_from_slice(&other.data);
        Vector { data }
    }

    /// Efficiently stacks multiple [`VectorData`] into one.
    #[inline]
    pub fn stack(vectors: &[&Self]) -> Self {
        let total_len: usize = vectors.iter().map(|v| v.data.len()).sum();

        let mut data = Vec::with_capacity(total_len);

        let mut ptr = data.as_mut_ptr();
        for v in vectors {
            let len = v.data.len();
            unsafe {
                std::ptr::copy_nonoverlapping(v.data.as_ptr(), ptr, len);
                ptr = ptr.add(len);
            }
        }
        unsafe { data.set_len(total_len) };
        Self { data }
    }
}