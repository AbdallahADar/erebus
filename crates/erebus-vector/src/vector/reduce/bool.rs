// === Imports ===
use crate::prelude::*;

// === Impl ===

impl Vector<bool> {

    /// Any: true if ANY valid element is true.
    #[inline]
    pub fn any(&self) -> bool {
        for &v in &self.data {
            if v {
                return true; // early stop
            }
        }
        false
    }

    /// All: true if ALL valid elements are true.
    #[inline]
    pub fn all(&self) -> bool {
        for &v in &self.data {
            if !v {
                return false; // early stop
            }
        }
        true
    }

    /// Count all true values.
    #[inline]
    pub fn count_true(&self) -> usize {
        let mut cnt = 0usize;
        for &v in &self.data {
            if v { cnt += 1; }
        }
        cnt
    }

    /// Count all false values.
    #[inline]
    pub fn count_false(&self) -> usize {
        let mut cnt = 0usize;
        for &v in &self.data {
            if !v { cnt += 1; }
        }
        cnt
    }

}