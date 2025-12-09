// === Imports ===
use crate::prelude::*;

// === Impl ===

impl VectorData<bool> {

    /// All: true if ALL valid elements are true.
    #[inline]
    pub fn all(&self) -> bool {
        let n = self.data.len();
        for i in 0..n {
            if unsafe { *self.validity.get_unchecked(i) } {
                if unsafe { !*self.data.get_unchecked(i) } {
                    return false; // early stop
                }
            }
        }
        true
    }

    /// Any: true if ANY valid element is true.
    #[inline]
    pub fn any(&self) -> bool {
        let n = self.data.len();
        for i in 0..n {
            if unsafe { *self.validity.get_unchecked(i) } {
                if unsafe { *self.data.get_unchecked(i) } {
                    return true; // early stop
                }
            }
        }
        false
    }

    /// Count valid true values.
    #[inline]
    pub fn count_true(&self) -> usize {
        let n = self.data.len();
        let mut cnt = 0usize;

        for i in 0..n {
            unsafe {
                if *self.validity.get_unchecked(i) &&
                   *self.data.get_unchecked(i)
                {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    /// Count valid false values.
    #[inline]
    pub fn count_false(&self) -> usize {
        let n = self.data.len();
        let mut cnt = 0usize;

        for i in 0..n {
            unsafe {
                if *self.validity.get_unchecked(i) &&
                   !*self.data.get_unchecked(i)
                {
                    cnt += 1;
                }
            }
        }
        cnt
    }

}