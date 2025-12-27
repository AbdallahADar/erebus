// === Imports ===
use crate::prelude::*;
use super::convert::{ymd_to_days, days_to_ymd};

// === Impls ===

impl Date {
    /// Extract year component.
    #[inline]
    pub fn year(&self) -> i32 {
        days_to_ymd(self.days).0
    }

    /// Extract month component (1–12).
    #[inline]
    pub fn month(&self) -> u8 {
        days_to_ymd(self.days).1
    }

    /// Extract day-of-month component (1–31).
    #[inline]
    pub fn day(&self) -> u8 {
        days_to_ymd(self.days).2
    }

    /// Extract (year, month, day).
    #[inline]
    pub fn ymd(&self) -> (i32, u8, u8) {
        days_to_ymd(self.days)
    }
}

impl DateVector {
    /// Extract years vector.
    pub fn years(&self) -> Vec<i32> {
        let n = self.days.len();
        let mut out = Vec::<i32>::with_capacity(n);

        unsafe {
            out.set_len(n);
            for i in 0..n {
                let v = if *self.validity.get_unchecked(i) {
                    days_to_ymd(*self.days.get_unchecked(i)).0
                } else {
                    0
                };
                *out.get_unchecked_mut(i) = v;
            }
        }

        out
    }

    /// Extract months vector (1–12).
    pub fn months(&self) -> Vec<u8> {
        let n = self.days.len();
        let mut out = Vec::<u8>::with_capacity(n);

        unsafe {
            out.set_len(n);
            for i in 0..n {
                let v = if *self.validity.get_unchecked(i) {
                    days_to_ymd(*self.days.get_unchecked(i)).1
                } else {
                    0
                };
                *out.get_unchecked_mut(i) = v;
            }
        }

        out
    }

    /// Extract days-of-month vector (1–31).
    pub fn days(&self) -> Vec<u8> {
        let n = self.days.len();
        let mut out = Vec::<u8>::with_capacity(n);

        unsafe {
            out.set_len(n);
            for i in 0..n {
                let v = if *self.validity.get_unchecked(i) {
                    days_to_ymd(*self.days.get_unchecked(i)).2
                } else {
                    0
                };
                *out.get_unchecked_mut(i) = v;
            }
        }

        out
    }

    /// Extract (year, month, day) tuples.
    pub fn ymd(&self) -> Vec<(i32, u8, u8)> {
        let n = self.days.len();
        let mut out = Vec::<(i32, u8, u8)>::with_capacity(n);

        unsafe {
            out.set_len(n);
            for i in 0..n {
                let v = if *self.validity.get_unchecked(i) {
                    days_to_ymd(*self.days.get_unchecked(i))
                } else {
                    (0, 0, 0)
                };
                *out.get_unchecked_mut(i) = v;
            }
        }

        out
    }
}