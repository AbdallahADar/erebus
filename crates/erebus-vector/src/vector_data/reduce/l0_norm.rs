// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone, Default)]
pub struct L0Acc {
    pub count: usize,
}

#[derive(Clone)]
pub struct L0NormReducer;

impl<T: Numeric> Reducer<T> for L0NormReducer {
    type Acc = L0Acc;
    type Output = usize;

    #[inline]
    fn accumulate(&mut self, acc: &mut L0Acc, value: &T, is_valid: bool) {
        if is_valid && *value != T::zero() {
            acc.count += 1;
        }
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        a.count += b.count;
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> usize {
        acc.count
    }
}

impl<T: Numeric> VectorData<T> {
    #[inline]
    pub fn l0_norm(&self) -> usize {
        self._reduce(L0NormReducer)
    }
}