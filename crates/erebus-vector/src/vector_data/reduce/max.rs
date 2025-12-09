// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone)]
pub struct MaxAcc<T> {
    pub max_val: Option<T>,
    pub max_idx: Option<usize>,
}

impl<T> Default for MaxAcc<T> {
    #[inline]
    fn default() -> Self {
        Self { max_val: None, max_idx: None }
    }
}

#[derive(Clone)]
pub struct MaxReducer;

impl<T> ReducerIndexed<T> for MaxReducer
where
    T: Numeric + Copy + PartialOrd,
{
    type Acc = MaxAcc<T>;
    type Output = MaxAcc<T>;

    #[inline]
    fn accumulate(
        &mut self,
        acc: &mut Self::Acc,
        value: &T,
        is_valid: bool,
        idx: usize,
    ) {
        if !is_valid {
            return;
        }

        match acc.max_val {
            None => {
                // First valid value seen
                acc.max_val = Some(*value);
                acc.max_idx = Some(idx);
            }
            Some(cur) => {
                // Compare against current max
                if value.partial_cmp(&cur) == Some(Ordering::Greater) {
                    acc.max_val = Some(*value);
                    acc.max_idx = Some(idx);
                }
            }
        }
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        match (a.max_val, b.max_val) {
            (None, None) => {}
            (None, Some(_)) => {
                // all of 'a' are null, copy b
                a.max_val = b.max_val;
                a.max_idx = b.max_idx;
            }
            (Some(_), None) => {
                // b had no valid entries → keep a
            }
            (Some(av), Some(bv)) => {
                if bv > av {
                    a.max_val = Some(bv);
                    a.max_idx = b.max_idx;
                }
            }
        }
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> Self::Output {
        acc
    }
}

impl<T: Numeric + Copy + PartialOrd> VectorData<T> {
    #[inline]
    pub fn max(&self) -> Option<T> {
        self._reduce_indexed(MaxReducer).max_val
    }

    #[inline]
    pub fn argmax(&self) -> Option<usize> {
        self._reduce_indexed(MaxReducer).max_idx
    }

    #[inline]
    pub fn max_with_index(&self) -> (Option<T>, Option<usize>) {
        let out = self._reduce_indexed(MaxReducer);
        (out.max_val, out.max_idx)
    }
}