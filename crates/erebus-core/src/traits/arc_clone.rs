// === Imports ===
use std::sync::Arc;

// === Impl ===

/// Trait for providing consistent deep and shallow Arc-based cloning
/// across Erebus data structures.
/// - `arc_deep_clone()` performs a deep copy and wraps in a new `Arc`
/// - `arc_shallow_clone()` increments Arc reference count (O(1))
pub trait ArcClone: Clone + Sized {

    /// Deep clone: duplicates the underlying buffers and wraps in a new `Arc`.
    #[inline]
    fn arc_deep_clone(&self) -> Arc<Self> {
        Arc::new(self.clone())
    }

    /// Shallow clone: increments the Arc reference count (cheap share).
    #[inline]
    fn arc_shallow_clone(arc: &Arc<Self>) -> Arc<Self> {
        Arc::clone(arc)
    }
}