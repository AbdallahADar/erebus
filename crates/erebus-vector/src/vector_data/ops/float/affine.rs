// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<f64> {
    // -- Multiply then Add (x * a + b) --
    impl_unary_op!(
        params, inplace,
        mul_add, mul_add_inplace, mul_add_range,
        (a: f64, b: f64) -> (a, b),
        f64,
        |x: &f64, a: f64, b: f64| a * (*x) + b,
        |x: &mut f64, a: f64, b: f64| { *x = a * (*x) + b; }
    );

    // -- Multiply Scalar --
    impl_unary_op!(
        params, inplace,
        mul_scalar, mul_scalar_inplace, mul_scalar_range,
        (c: f64) -> (c),
        f64,
        |x: &f64, c: f64| (*x) * c,
        |x: &mut f64, c: f64| { *x *= c; }
    );

    // -- Add Scalar --
    impl_unary_op!(
        params, inplace,
        add_scalar, add_scalar_inplace, add_scalar_range,
        (c: f64) -> (c),
        f64,
        |x: &f64, c: f64| (*x) + c,
        |x: &mut f64, c: f64| { *x += c; }
    );

    // -- Subtract Scalar --
    impl_unary_op!(
        params, inplace,
        sub_scalar, sub_scalar_inplace, sub_scalar_range,
        (c: f64) -> (c),
        f64,
        |x: &f64, c: f64| (*x) - c,
        |x: &mut f64, c: f64| { *x -= c; }
    );

    // -- Divide Scalar --
    impl_unary_op!(
        params_valid, inplace,
        div_scalar, div_scalar_inplace, div_scalar_range,
        (c: f64) -> (c),
        f64,
        |x: &f64, c: f64| {
            let y = *x / c;
            if y.is_nan() { (0_f64, false) } else { (y, true) }
        },
        |x: &mut f64, c: f64| {
            let y = *x / c;
            if y.is_nan() {
                false
            } else {
                *x = y;
                true
            }
        }
    );

    // -- Inverse Divide Scalar --
    impl_unary_op!(
        params_valid, inplace,
        inv_div_scalar, inv_div_scalar_inplace, inv_div_scalar_range,
        (c: f64) -> (c),
        f64,
        |x: &f64, c: f64| {
            let y = c / *x;
            if y.is_nan() { (0_f64, false) } else { (y, true) }
        },
        |x: &mut f64, c: f64| {
            let y = c / *x;
            if y.is_nan() {
                false
            } else {
                *x = y;
                true
            }
        }
    );
}