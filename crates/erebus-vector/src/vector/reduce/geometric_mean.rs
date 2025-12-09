// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone, Default)]
pub struct GeoAcc {
    pub sum_log: f64,
    pub count: usize,
}

#[derive(Clone)]
pub struct GeometricMeanReducer;

impl<T: Numeric> Reducer<T> for GeometricMeanReducer {
    type Acc = GeoAcc;
    type Output = f64;

    #[inline]
    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, _is_valid: bool) {
        let v = value.to_f64();

        // Geometric mean undefined for nonpositive values
        if v <= 0.0 {
            acc.sum_log = f64::NAN;
            acc.count = 0;
            return;
        }

        acc.sum_log += v.ln();
        acc.count += 1;
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        a.sum_log += b.sum_log;
        a.count += b.count;
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> f64 {
        if acc.count == 0 || acc.sum_log.is_nan() {
            return f64::NAN;
        }
        (acc.sum_log / acc.count as f64).exp()
    }
}

impl<T: Numeric> Vector<T> {
    #[inline]
    pub fn geometric_mean(&self) -> f64 {
        self._reduce(GeometricMeanReducer)
    }
}