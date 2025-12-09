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

    #[inline]
    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, _is_valid: bool) {
        acc.sum_abs += value.abs();      // same logic as VectorData, but no validity
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        a.sum_abs += b.sum_abs;
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> T {
        acc.sum_abs
    }
}

impl<T: Numeric> Vector<T> {
    #[inline]
    pub fn l1_norm(&self) -> T {
        self._reduce(L1Reducer)
    }

    #[inline]
    pub fn sum_abs(&self) -> T {
        self.l1_norm()
    }
}