// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<f64> {
    // -- Ceil --
    impl_unary_op!(
        noparams, inplace,
        ceil, ceil_inplace, ceil_range,
        f64,
        |x: &f64| x.ceil(),
        |x: &mut f64| *x = x.ceil()
    );

    // -- Floor --
    impl_unary_op!(
        noparams, inplace,
        floor, floor_inplace, floor_range,
        f64,
        |x: &f64| x.floor(),
        |x: &mut f64| *x = x.floor()
    );

    // -- Round --
    impl_unary_op!(
        noparams, inplace,
        round, round_inplace, round_range,
        f64,
        |x: &f64| x.round(),
        |x: &mut f64| *x = x.round()
    );

    // -- Fractional --
    impl_unary_op!(
        noparams, inplace,
        fract, fract_inplace, fract_range,
        f64,
        |x: &f64| x.fract(),
        |x: &mut f64| *x = x.fract()
    );

    // -- Truncate --
    impl_unary_op!(
        noparams, inplace,
        trunc, trunc_inplace, trunc_range,
        f64,
        |x: &f64| x.trunc(),
        |x: &mut f64| *x = x.trunc()
    );

    // -- Round Up: Just an alias for Ceil --
    #[inline]
    pub fn roundup(&self) -> VectorData<f64> {
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
    ) -> VectorData<f64> {
        self.ceil_range(start, end, full)
    }

    // -- Clip (min ≤ x ≤ max) --
    impl_unary_op!(
        params_valid, inplace,
        clip, clip_inplace, clip_range,
        (lo: f64, hi: f64) -> (lo, hi),
        f64,
        |x: &f64, lo: f64, hi: f64| {
            if lo > hi {
                (0.0, false)
            } else if *x < lo {
                (lo, true)
            } else if *x > hi {
                (hi, true)
            } else {
                (*x, true)
            }
        },
        |x: &mut f64, lo: f64, hi: f64| {
            if lo > hi {
                *x = 0.0;
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
    pub fn clamp(&self, lo: f64, hi: f64) -> VectorData<f64> {
        self.clip(lo, hi)
    }
    #[inline]
    pub fn clamp_inplace(&mut self, lo: f64, hi: f64) {
        self.clip_inplace(lo, hi)
    }
    #[inline]
    pub fn clamp_range(
        &self,
        lo: f64,
        hi: f64,
        start: usize,
        end: usize,
        full: bool,
    ) -> VectorData<f64> {
        self.clip_range(lo, hi, start, end, full)
    }

}