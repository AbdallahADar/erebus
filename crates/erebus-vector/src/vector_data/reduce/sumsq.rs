// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone, Default)]
pub struct SumSqAcc {
    pub sumsq: f64,
}

#[derive(Clone)]
pub struct SumSqReducer;

impl<T: Numeric> Reducer<T> for SumSqReducer {
    type Acc = SumSqAcc;
    type Output = f64;

    #[inline]
    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, is_valid: bool) {
        if is_valid {
            let v = value.to_f64();
            acc.sumsq += v * v;
        }
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        a.sumsq += b.sumsq;
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> f64 {
        acc.sumsq
    }
}

impl<T: Numeric> VectorData<T> {
    #[inline]
    pub fn sum_of_squares(&self) -> f64 {
        self._reduce(SumSqReducer)
    }
}