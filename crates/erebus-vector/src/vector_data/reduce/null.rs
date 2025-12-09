// === Imports ===
use crate::prelude::*;

// === Impl ===

impl<T: Clone + Send + Sync + 'static> VectorData<T> {

    #[inline]
    pub fn null_ratio(&self) -> f64 {
        let n = self.data.len();
        if n == 0 {
            return 0.0;
        }
        let invalid = n - self.validity.count_ones();
        invalid as f64 / n as f64
    }

    #[inline]
    pub fn non_null_ratio(&self) -> f64 {
        let n = self.data.len();
        if n == 0 {
            return 1.0;
        }
        let valid = self.validity.count_ones();
        valid as f64 / n as f64
    }

    #[inline]
    pub fn null_percentage(&self) -> f64 {
        self.null_ratio() * 100.0
    }

    #[inline]
    pub fn non_null_percentage(&self) -> f64 {
        self.non_null_ratio() * 100.0
    }
}