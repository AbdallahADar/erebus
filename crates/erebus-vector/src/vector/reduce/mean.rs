// === Imports ===
use crate::prelude::*;

// === Impl ===

impl Vector<i64> {
    #[inline]
    pub fn mean(&self) -> f64 {
        let n = self.data.len();
        if n == 0 {
            return f64::NAN;
        }
        let sum: i64 = self.data.iter().copied().sum();
        sum as f64 / n as f64
    }
}

impl Vector<f64> {
    #[inline]
    pub fn mean(&self) -> f64 {
        let n = self.data.len();
        if n == 0 {
            return f64::NAN;
        }
        let sum: f64 = self.data.iter().copied().sum();
        sum / n as f64
    }
}