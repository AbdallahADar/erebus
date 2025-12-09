// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone, Default)]
pub struct MaxAcc {
    pub max_abs: f64,
}

#[derive(Clone)]
pub struct MaxNormReducer;

impl<T: Numeric> Reducer<T> for MaxNormReducer {
    type Acc = MaxAcc;
    type Output = f64;

    #[inline]
    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, is_valid: bool) {
        if is_valid {
            let v = value.to_f64().abs();
            if v > acc.max_abs {
                acc.max_abs = v;
            }
        }
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        if b.max_abs > a.max_abs {
            a.max_abs = b.max_abs;
        }
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> Self::Output {
        acc.max_abs
    }
}

impl<T: Numeric> VectorData<T> {

    #[inline]
    pub fn linf_norm(&self) -> f64 {
        self._reduce(MaxNormReducer)
    }
}