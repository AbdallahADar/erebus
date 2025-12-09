// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone, Default)]
pub struct SkewAcc {
    pub n: usize,
    pub sum1: f64,
    pub sum2: f64,
    pub sum3: f64,
}

#[derive(Clone)]
pub struct SkewnessReducer;

impl<T: Numeric> Reducer<T> for SkewnessReducer {
    type Acc = SkewAcc;
    type Output = f64;

    #[inline]
    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, is_valid: bool) {
        if is_valid {
            let v = value.to_f64();
            acc.n += 1;
            acc.sum1 += v;
            acc.sum2 += v * v;
            acc.sum3 += v * v * v;
        }
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        a.n += b.n;
        a.sum1 += b.sum1;
        a.sum2 += b.sum2;
        a.sum3 += b.sum3;
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> f64 {
        if acc.n < 2 { return 0.0; }

        let n = acc.n as f64;
        let mean = acc.sum1 / n;

        let m2 = acc.sum2 / n - mean * mean;
        if m2 == 0.0 { return 0.0; }

        // Third central moment
        let m3 = acc.sum3 / n - 3.0 * mean * (acc.sum2 / n) + 2.0 * mean.powi(3);

        m3 / m2.powf(1.5)
    }
}

impl<T: Numeric> VectorData<T> {
    #[inline]
    pub fn skewness(&self) -> f64 {
        self._reduce(SkewnessReducer)
    }
}