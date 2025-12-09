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
}