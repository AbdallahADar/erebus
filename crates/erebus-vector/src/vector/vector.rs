// === Imports ===
use crate::prelude::*;

// === Types ===

/// Simple dense vector structure (no nulls, no validity bitmap).
#[derive(Debug, Clone, PartialEq)]
pub struct Vector<T> {
    pub data: Vec<T>,
}

// === Impl ===

impl<T: Clone + Default> Vector<T> {

    /// Create an empty Vector.
    #[inline]
    pub fn empty() -> Self {
        Self { data: Vec::new() }
    }

    /// Alias of `empty`
    pub fn new() -> Self { Self::empty() }

    /// Create a Vector filled with `n` copies of `value`.
    #[inline]
    pub fn full(value: T, n: usize) -> Self {
        Self { data: vec![value; n] }
    }

    /// # Safety
    /// Constructs a [`Vector`] from an existing `Vec` **without validation**.
    /// - Skips the emptiness check.
    /// - Intended for internal or performance-critical usage.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _from_vec(data: Vec<T>) -> Self {
        debug_assert!(
            !data.is_empty(),
            "Vector::_from_vec called with empty Vec"
        );
        Self { data }
    }

    /// Constructs a new [`Vector`] safely from an existing `Vec`.
    /// Returns an error if the provided vector is empty.
    #[inline]
    pub fn from_vec(data: Vec<T>) -> Result<Self, ErebusError> {
        if data.is_empty() {
            return Err(ErebusError::EmptyVector);
        }
        Ok(Self::_from_vec(data))
    }

    /// Return length of the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Track memory usage (rough estimate).
    #[inline]
    pub fn memory_usage(&self) -> usize {
        std::mem::size_of_val(&self.data)
            + self.data.capacity() * std::mem::size_of::<T>()
    }

}

impl<T: Clone> From<VectorData<T>> for Vector<T> {
    fn from(vd: VectorData<T>) -> Self {
        let data = vd.data
            .into_iter()
            .zip(vd.validity.into_iter())
            .filter_map(|(value, valid)| if valid { Some(value) } else { None })
            .collect();
        Self { data }
    }
}

impl<T: std::fmt::Display> Vector<T> {
    /// Print the vector as a readable string.
    pub fn print(&self) -> String {
        let out = self.data.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{}]", out)
    }
}

impl<'a, T: 'a> Viewable<'a, T> for Vector<T> {
    type ViewType = VectorView<'a, T>;

    #[inline]
    fn view(&'a self) -> Self::ViewType {
        VectorView { data: &self.data }
    }
}