// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<i64> {
    // -- Ceil --
    impl_unary_op!(
        noparams, inplace,
        ceil, ceil_inplace, ceil_range,
        i64,
        |x: &i64| *x,
        |_: &mut i64| {}
    );

    // -- Floor --
    impl_unary_op!(
        noparams, inplace,
        floor, floor_inplace, floor_range,
        i64,
        |x: &i64| *x,
        |_: &mut i64| {}
    );

    // -- Round --
    impl_unary_op!(
        noparams, inplace,
        round, round_inplace, round_range,
        i64,
        |x: &i64| *x,
        |_: &mut i64| {}
    );


    // -- Fractional --
    impl_unary_op!(
        noparams, inplace,
        fract, fract_inplace, fract_range,
        i64,
        |_x: &i64| 0_i64,
        |x: &mut i64| *x = 0_i64
    );

    // -- Truncate --
    impl_unary_op!(
        noparams, inplace,
        trunc, trunc_inplace, trunc_range,
        i64,
        |x: &i64| *x,
        |_: &mut i64| {}
    );

    // -- Round Up: Just an alias for Ceil --
    #[inline]
    pub fn roundup(&self) -> VectorData<i64> {
        self.ceil()
    }

    #[inline]
    pub fn roundup_inplace(&mut self) {
        self.ceil_inplace();
    }

    #[inline]
    pub fn roundup_range(
        &self,
        start: usize,
        end: usize,
        full: bool,
    ) -> VectorData<i64> {
        self.ceil_range(start, end, full)
    }

    // -- Clip (min ≤ x ≤ max) --
    impl_unary_op!(
        params_valid, inplace,
        clip, clip_inplace, clip_range,
        (lo: i64, hi: i64) -> (lo, hi),
        i64,
        |x: &i64, lo: i64, hi: i64| {
            if lo > hi {
                (0, false)
            } else if *x < lo {
                (lo, true)
            } else if *x > hi {
                (hi, true)
            } else {
                (*x, true)
            }
        },
        |x: &mut i64, lo: i64, hi: i64| {
            if lo > hi {
                *x = 0;
                false
            } else if *x < lo {
                *x = lo;
                true
            } else if *x > hi {
                *x = hi;
                true
            } else {
                true
            }
        }
    );

    // -- Clamp (alias for Clip) --
    #[inline]
    pub fn clamp(&self, lo: i64, hi: i64) -> VectorData<i64> {
        self.clip(lo, hi)
    }
    #[inline]
    pub fn clamp_inplace(&mut self, lo: i64, hi: i64) {
        self.clip_inplace(lo, hi);
    }
    #[inline]
    pub fn clamp_range(
        &self,
        lo: i64,
        hi: i64,
        start: usize,
        end: usize,
        full: bool,
    ) -> VectorData<i64> {
        self.clip_range(lo, hi, start, end, full)
    }
}