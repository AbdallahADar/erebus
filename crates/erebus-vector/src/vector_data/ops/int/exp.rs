// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<i64> {

    // -- Exponential --
    impl_unary_op!(
        noparams, noinplace,
        exp, exp_range,
        f64,
        |x: &i64| (*x as f64).exp()
    );

    // -- Exponentnial Minus 1 --
    impl_unary_op!(
        noparams, noinplace,
        exp_m1, exp_m1_range,
        f64,
        |x: &i64| (*x as f64).exp_m1()
    );
}