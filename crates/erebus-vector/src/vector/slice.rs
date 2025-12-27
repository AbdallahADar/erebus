// === Imports ===
use crate::prelude::*;

// === Impl ===

impl<T: Clone + Send + Sync + 'static> Vector<T> {
    /// Returns an owned copy of a slice in range `[start, end)`.
    #[inline]
    pub fn slice(&self, start: usize, end: usize) -> Self {
        if start >= end || start >= self.data.len() {
            return Self { data: Vec::new() };
        }

        let end = end.min(self.data.len());
        Self {
            data: self.data[start..end].to_vec(),
        }
    }

    /// Returns a zero-copy borrowed view of the range `[start, end)`.
    #[inline]
    pub fn slice_view(&self, start: usize, end: usize) -> VectorView<'_, T> {
        let len = self.data.len();
        if start >= end || start >= len {
            return VectorView { data: &[] };
        }

        let end = end.min(len);
        VectorView { data: &self.data[start..end] }
    }
}