// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone, Default)]
pub struct LpAcc {
    pub sum_p: f64,
}

/// Reducer for Lp norm over Vector<T>
#[derive(Clone)]
pub struct LpNormReducer {
    pub p: f64,
}

impl<T: Numeric> Reducer<T> for LpNormReducer {
    type Acc = LpAcc;
    type Output = f64;

    #[inline]
    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, _is_valid: bool) {
        let v = value.to_f64().abs();
        acc.sum_p += v.powf(self.p);
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        a.sum_p += b.sum_p;
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> f64 {
        // p must be > 0 and finite
        if !self.p.is_finite() || self.p <= 0.0 {
            return f64::NAN;
        }

        if acc.sum_p == 0.0 {
            return 0.0; // convention
        }

        acc.sum_p.powf(1.0 / self.p)
    }
}

impl<T: Numeric> Vector<T> {
    /// General Lp norm: ( Σ |x|^p )^(1/p)
    #[inline]
    pub fn lp_norm(&self, p: f64) -> f64 {
        self._reduce(LpNormReducer { p })
    }

    /// L2 norm: special case of Lp with p = 2.
    #[inline]
    pub fn l2_norm(&self) -> f64 {
        self.lp_norm(2.0)
    }
}