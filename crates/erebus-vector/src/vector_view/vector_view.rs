// === Imports ===


// === Types ===

/// A zero-copy borrowed view of a non-nullable [`Vector<T>`].
/// Equivalent to an `&[T]` slice, but with room for API extensions.
#[derive(Debug, Clone, Copy)]
pub struct VectorView<'a, T> {
    pub data: &'a [T],
}

// === Impl ===

impl<'a, T> VectorView<'a, T> {

    /// Number of elements in this view.
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns element at index `i`.
    #[inline]
    pub fn get(&self, i: usize) -> Option<&'a T> {
        self.data.get(i)
    }

    /// Iterates all elements.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &'a T> {
        self.data.iter()
    }
}