// === Imports ===
use crate::prelude::*;

// === Impl ===

impl<T: Clone + Send + Sync + 'static> Vector<T> {

    #[inline]
    pub fn null_ratio(&self) -> f64 {
        0.0
    }

    #[inline]
    pub fn non_null_ratio(&self) -> f64 {
        1.0
    }

    #[inline]
    pub fn null_percentage(&self) -> f64 {
        0.0
    }

    #[inline]
    pub fn non_null_percentage(&self) -> f64 {
        100.0
    }
}