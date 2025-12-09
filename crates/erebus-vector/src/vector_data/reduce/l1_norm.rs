// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone, Default)]
pub struct L1Acc<T: Numeric> {
    pub sum_abs: T,
}

#[derive(Clone)]
pub struct L1Reducer;

impl<T: Numeric> Reducer<T> for L1Reducer {
    type Acc = L1Acc<T>;
    type Output = T;

    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, is_valid: bool) {
        if is_valid {
            acc.sum_abs += value.abs();     // using Numeric::abs()
        }
    }

    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        a.sum_abs += b.sum_abs;
    }

    fn finalize(&self, acc: Self::Acc) -> Self::Output {
        acc.sum_abs
    }
}

impl<T: Numeric> VectorData<T> {
    #[inline]
    pub fn l1_norm(&self) -> T {
        self._reduce(L1Reducer)
    }

    #[inline]
    pub fn sum_abs(&self) -> T {
        self.l1_norm()
    }
}