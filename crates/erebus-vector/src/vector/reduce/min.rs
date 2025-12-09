// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone)]
pub struct MinAcc<T> {
    pub min_val: Option<T>,
    pub min_idx: Option<usize>,
}

impl<T> Default for MinAcc<T> {
    #[inline]
    fn default() -> Self {
        Self { min_val: None, min_idx: None }
    }
}

#[derive(Clone)]
pub struct minReducer;

impl<T> ReducerIndexed<T> for minReducer
where
    T: Numeric + Clone + PartialOrd,
{
    type Acc = MinAcc<T>;
    type Output = MinAcc<T>;

    #[inline]
    fn accumulate(
        &mut self,
        acc: &mut Self::Acc,
        value: &T,
        is_valid: bool,
        idx: usize,
    ) {
        match &acc.min_val {
            None => {
                acc.min_val = Some(value.clone());
                acc.min_idx = Some(idx);
            }
            Some(current) => {
                if value.partial_cmp(current) == Some(Ordering::Less) {
                    acc.min_val = Some(value.clone());
                    acc.min_idx = Some(idx);
                }
            }
        }
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        match (&a.min_val, b.min_val) {
            (None, None) => {}
            (None, Some(bv)) => {
                a.min_val = Some(bv);
                a.min_idx = b.min_idx;
            }
            (Some(_), None) => {}
            (Some(av), Some(bv)) => {
                if bv.partial_cmp(av) == Some(Ordering::Less) {
                    a.min_val = Some(bv);
                    a.min_idx = b.min_idx;
                }
            }
        }
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> Self::Output {
        acc
    }
}

impl<T> Vector<T>
where
    T: Numeric + Copy + PartialOrd,
{
    #[inline]
    pub fn min(&self) -> Option<T> {
        self._reduce_indexed(minReducer).min_val
    }

    #[inline]
    pub fn argmin(&self) -> Option<usize> {
        self._reduce_indexed(minReducer).min_idx
    }

    /// Combined convenience form — returns `(min_value, index)`
    #[inline]
    pub fn min_with_index(&self) -> (Option<T>, Option<usize>) {
        let out = self._reduce_indexed(minReducer);
        (out.min_val, out.min_idx)
    }
}