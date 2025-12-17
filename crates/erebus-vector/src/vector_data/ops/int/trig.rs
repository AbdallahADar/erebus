// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<i64> {

    // -- Sin --
    impl_unary_op!(
        noparams, noinplace,
        sin, sin_range,
        f64,
        |x: &i64| (*x as f64).sin()
    );

    // -- Cos --
    impl_unary_op!(
        noparams, noinplace,
        cos, cos_range,
        f64,
        |x: &i64| (*x as f64).cos()
    );

    // -- Tan --
    impl_unary_op!(
        noparams_valid, noinplace,
        tan, tan_range,
        f64,
        |x: &i64| {
            let y = (*x as f64).tan();
            if y.is_finite() {
                (y, true)
            } else {
                (0.0, false)
            }
        }
    );
}