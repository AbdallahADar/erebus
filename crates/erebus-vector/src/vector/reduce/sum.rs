// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone, Default)]
pub struct SumAcc<T: Numeric> {
    pub sum: T,
}

#[derive(Clone)]
pub struct SumReducer;

impl<T: Numeric> Reducer<T> for SumReducer {
    type Acc = SumAcc<T>;
    type Output = T;

    #[inline]
    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, _is_valid: bool) {
        // Vector assumes everything is valid
        acc.sum += *value;
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        a.sum += b.sum;
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> Self::Output {
        acc.sum
    }
}

impl<T: Numeric> Vector<T> {
    #[inline]
    pub fn sum(&self) -> T {
        self._reduce(SumReducer)
    }
}