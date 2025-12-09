// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<i64> {
    // -- Absolute --
    impl_unary_op!(
        noparams, inplace,
        abs, abs_inplace, abs_range,
        i64,
        |x: &i64| x.abs(),
        |x: &mut i64| *x = x.abs()
    );

    // -- Signum --
    impl_unary_op!(
        noparams, inplace,
        signum, signum_inplace, signum_range,
        i64,
        |x: &i64| x.signum(),
        |x: &mut i64| *x = x.signum()
    );

    // -- Negate --
    impl_unary_op!(
        noparams, inplace,
        neg, neg_inplace, neg_range,
        i64,
        |x: &i64| -*x,
        |x: &mut i64| *x = -*x
    );

    // -- Flip Sign (Alias for Negate) --
    impl_unary_op!(
        noparams, inplace,
        flip_sign, flip_sign_inplace, flip_sign_range,
        i64,
        |x: &i64| -*x,
        |x: &mut i64| *x = -*x
    );

    // -- SignBit return True for negatives --
    impl_unary_op!(
        noparams, noinplace,
        signbit, signbit_range,
        bool,
        |x: &i64| *x < 0
    );
}