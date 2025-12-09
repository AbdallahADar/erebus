// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone, Default)]
pub struct KurtAcc {
    pub n: usize,
    pub sum1: f64,
    pub sum2: f64,
    pub sum3: f64,
    pub sum4: f64,
}

#[derive(Clone)]
pub struct KurtosisReducer;

impl<T: Numeric> Reducer<T> for KurtosisReducer {
    type Acc = KurtAcc;
    type Output = f64;

    #[inline]
    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, is_valid: bool) {
        if is_valid {
            let v = value.to_f64();
            acc.n += 1;
            acc.sum1 += v;
            acc.sum2 += v * v;
            acc.sum3 += v * v * v;
            acc.sum4 += v * v * v * v;
        }
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        a.n += b.n;
        a.sum1 += b.sum1;
        a.sum2 += b.sum2;
        a.sum3 += b.sum3;
        a.sum4 += b.sum4;
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> f64 {
        if acc.n < 2 { return 0.0; }

        let n = acc.n as f64;
        let mean = acc.sum1 / n;

        let e2 = acc.sum2 / n;
        let e3 = acc.sum3 / n;
        let e4 = acc.sum4 / n;

        // Central moments
        let m2 = e2 - mean * mean;
        if m2 == 0.0 { return 0.0; }

        let m4 =
            e4 - 4.0 * mean * e3 +
            6.0 * mean * mean * e2 -
            3.0 * mean.powi(4);

        m4 / (m2 * m2) - 3.0
    }
}

impl<T: Numeric> VectorData<T> {
    #[inline]
    pub fn kurtosis(&self) -> f64 {
        self._reduce(KurtosisReducer)
    }
}