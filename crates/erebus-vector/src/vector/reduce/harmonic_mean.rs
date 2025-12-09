// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone, Default)]
pub struct HarmAcc {
    pub sum_recip: f64,
    pub count: usize,
}

#[derive(Clone)]
pub struct HarmonicMeanReducer;

impl<T: Numeric> Reducer<T> for HarmonicMeanReducer {
    type Acc = HarmAcc;
    type Output = f64;

    #[inline]
    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, _is_valid: bool) {
        let v = value.to_f64();

        if v == 0.0 {
            // Harmonic mean undefined if any value is zero
            acc.sum_recip = f64::NAN;
            return;
        }

        acc.sum_recip += 1.0 / v;
        acc.count += 1;
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        a.sum_recip += b.sum_recip;
        a.count += b.count;
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> f64 {
        if acc.sum_recip.is_nan() || acc.count == 0 {
            return f64::NAN;
        }
        (acc.count as f64) / acc.sum_recip
    }
}

impl<T: Numeric> Vector<T> {
    #[inline]
    pub fn harmonic_mean(&self) -> f64 {
        self._reduce(HarmonicMeanReducer)
    }
}