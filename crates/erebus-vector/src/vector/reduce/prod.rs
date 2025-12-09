// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone)]
pub struct ProdAcc<T: Numeric> {
    pub prod: T,
}

impl<T: Numeric> Default for ProdAcc<T> {
    #[inline]
    fn default() -> Self {
        // Multiplicative identity
        Self { prod: T::one() }
    }
}

#[derive(Clone)]
pub struct ProdReducer;

impl<T: Numeric> Reducer<T> for ProdReducer {
    type Acc = ProdAcc<T>;
    type Output = T;

    #[inline]
    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, _is_valid: bool) {
        acc.prod *= *value;
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        a.prod *= b.prod;
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> T {
        acc.prod
    }
}

impl<T: Numeric> Vector<T> {
    #[inline]
    pub fn prod(&self) -> T {
        self._reduce(ProdReducer)
    }
}