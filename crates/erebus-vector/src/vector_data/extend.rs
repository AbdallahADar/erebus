// === Imports ===
use crate::prelude::*;

// === Impl ===

impl<T: Clone + Send + Sync> VectorData<T> {

    /// Extends in place from a raw slice, marking all new values as valid.
    #[inline]
    pub fn extend(&mut self, slice: &[T]) {
        self.data.extend_from_slice(slice);
        self.validity.extend(std::iter::repeat(true).take(slice.len()));
    }

    /// Appends another `VectorData<T>` in place (copies both data and validity).
    #[inline]
    pub fn append(&mut self, other: &VectorData<T>) {
        self.data.extend_from_slice(&other.data);
        self.validity.extend(other.validity.iter());
    }

    /// Returns a new `VectorData` by concatenating two vectors.
    #[inline]
    pub fn concat(&self, other: &VectorData<T>) -> VectorData<T> {
        let mut data = Vec::with_capacity(self.data.len() + other.data.len());
        data.extend_from_slice(&self.data);
        data.extend_from_slice(&other.data);

        let mut validity = self.validity.clone();
        validity.extend(other.validity.iter());

        VectorData { data, validity }
    }

    /// Efficiently stacks multiple [`VectorData`] into one.
    #[inline]
    pub fn stack(vectors: &[&Self]) -> Self {
        let total_len: usize = vectors.iter().map(|v| v.data.len()).sum();

        let mut data = Vec::with_capacity(total_len);
        let mut validity = BitVec::with_capacity(total_len);

        let mut ptr = data.as_mut_ptr();
        for v in vectors {
            let len = v.data.len();
            unsafe { std::ptr::copy_nonoverlapping(v.data.as_ptr(), ptr, len) };
            ptr = unsafe { ptr.add(len) };
        }
        unsafe { data.set_len(total_len) };
        for v in vectors {
            validity.extend(v.validity.iter());
        }
        Self { data, validity }
    }

}