// === Imports ===
use crate::prelude::*;

// === Impl ===

impl<T: Clone + Send + Sync + 'static> VectorData<T> {

    /// Returns an owned slice copy in range `[start, end)`.
    /// Clamps range to valid bounds and preserves validity.
    #[inline]
    pub fn slice(&self, start: usize, end: usize) -> Self {
        if start >= end || start >= self.data.len() {
            return Self {
                data: Vec::new(),
                validity: bitvec![0; 0],
            };
        }

        let end = end.min(self.data.len());
        Self {
            data: self.data[start..end].to_vec(),
            validity: self.validity[start..end].to_bitvec(),
        }
    }

    /// Returns a zero-copy borrowed view over the range `[start, end)`.
    /// Performs no allocations; safe if parent `VectorData` remains alive.
    #[inline]
    pub fn slice_view(&self, start: usize, end: usize) -> VectorDataView<'_, T> {
        let len = self.data.len();
        if start >= end || start >= len {
            return VectorDataView {
                data: &[],
                validity: BitSlice::empty(),
            };
        }

        let end = end.min(len);
        VectorDataView {
            data: &self.data[start..end],
            validity: &self.validity[start..end],
        }
    }
}