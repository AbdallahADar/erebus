// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<i64> {

    // -- Multiply then Add (x * a + b) --
    impl_unary_op!(
        params, noinplace,
        mul_add, mul_add_range,
        (a: f64, b: f64) -> (a, b),
        f64,
        |x: &i64, a: f64, b: f64| a * (*x as f64) + b
    );

    // -- Multiply Scalar --
    impl_unary_op!(
        params, noinplace,
        mul_scalar, mul_scalar_range,
        (c: f64) -> (c),
        f64,
        |x: &i64, c: f64| (*x as f64) * c
    );

    // -- Add Scalar --
    impl_unary_op!(
        params, noinplace,
        add_scalar, add_scalar_range,
        (c: f64) -> (c),
        f64,
        |x: &i64, c: f64| (*x as f64) + c
    );

    // -- Subtract Scalar --
    impl_unary_op!(
        params, noinplace,
        sub_scalar, sub_scalar_range,
        (c: f64) -> (c),
        f64,
        |x: &i64, c: f64| (*x as f64) - c
    );

    // -- Divide Scalar --
    impl_unary_op!(
        params_valid, noinplace,
        div_scalar, div_scalar_range,
        (c: f64) -> (c),
        f64,
        |x: &i64, c: f64| {
            let y = (*x as f64)/c;
            if y.is_nan() { (y, false) } else { (y, true) }
        }
    );

    // -- Inverse Divide Scalar --
    impl_unary_op!(
        params_valid, noinplace,
        inv_div_scalar, inv_div_scalar_range,
        (c: f64) -> (c),
        f64,
        |x: &i64, c: f64| {
            let y = c / (*x as f64);
            if y.is_nan() { (y, false) } else { (y, true) }
        }
    );

}