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
}