// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<f64> {
    // -- Exponential --
    impl_unary_op!(
        noparams, inplace,
        exp, exp_inplace, exp_range,
        f64,
        |x: &f64| x.exp(),
        |x: &mut f64| *x = x.exp()
    );

    // -- Exponential Minus 1 --
    impl_unary_op!(
        noparams, inplace,
        exp_m1, exp_m1_inplace, exp_m1_range,
        f64,
        |x: &f64| x.exp_m1(),
        |x: &mut f64| *x = x.exp_m1()
    );
}