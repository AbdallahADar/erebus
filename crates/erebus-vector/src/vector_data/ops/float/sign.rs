// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<f64> {
    // -- Absolute --
    impl_unary_op!(
        noparams, inplace,
        abs, abs_inplace, abs_range,
        f64,
        |x: &f64| x.abs(),
        |x: &mut f64| *x = x.abs()
    );

    // -- Signum --
    // x.signum() does not return 0.0 for input 0.0
    // See this thread: https://github.com/rust-lang/rust/issues/57543
    // So we just implement a custom one
    impl_unary_op!(
        noparams, inplace,
        signum, signum_inplace, signum_range,
        f64,
        |x: &f64| {
            if *x > 0.0 { 1.0 }
            else if *x < 0.0 { -1.0 }
            else { 0.0 }
        },
        |x: &mut f64| {
            *x = if *x > 0.0 { 1.0 }
            else if *x < 0.0 { -1.0 }
            else { 0.0 };
        }
    );

    // -- Negate --
    impl_unary_op!(
        noparams, inplace,
        neg, neg_inplace, neg_range,
        f64,
        |x: &f64| -*x,
        |x: &mut f64| *x = -*x
    );

    // -- Flip Sign (Alias for Negate) --
    impl_unary_op!(
        noparams, inplace,
        flip_sign, flip_sign_inplace, flip_sign_range,
        f64,
        |x: &f64| -*x,
        |x: &mut f64| *x = -*x
    );

    // -- SignBit return True for negatives --
    impl_unary_op!(
        noparams, noinplace,
        signbit, signbit_range,
        bool,
        |x: &f64| x.is_sign_negative()
    );
}