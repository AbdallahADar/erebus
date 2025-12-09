// === Imports ===
use crate::prelude::*;

// === Impl ===

impl VectorData<i64> {
    pub fn mean(&self) -> f64 {
        let sum = self.sum() as f64;
        let count = self.validity.count_ones() as usize;
        if count == 0 { return f64::NAN; }
        sum / count as f64
    }
}

impl VectorData<f64> {
    pub fn mean(&self) -> f64 {
        let sum = self.sum();
        let count = self.validity.count_ones() as usize;
        if count == 0 { return f64::NAN; }
        sum / count as f64
    }
}