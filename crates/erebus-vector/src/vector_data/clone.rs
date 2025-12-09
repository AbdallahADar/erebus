// === Imports ===
use crate::prelude::*;

// === Impl ===

impl<T: Clone> VectorData<T> {
    /// Returns a completely new copy of this [`VectorData`],
    /// duplicating both the data buffer and the validity bitmap.
    /// This is equivalent to calling `.clone()` but provided as a more
    /// explicit alias to indicate intent.
    #[inline]
    pub fn deepcopy(&self) -> Self {
        self.clone()
    }
}

/// Integration with the shared [`ArcClone`] trait from `erebus-core`.
/// The [`ArcClone`] trait adds:
/// - `arc_deep_clone()` → deep copy wrapped in new Arc
/// - `arc_shallow_clone()` → bump Arc refcount (no data copy)
impl<T: Clone> ArcClone for VectorData<T> {}