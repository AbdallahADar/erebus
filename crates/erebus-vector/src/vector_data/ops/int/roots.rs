// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<i64> {

    // -- Square Root --
    impl_unary_op!(
        noparams_valid, noinplace,
        sqrt, sqrt_range,
        f64,
        |x: &i64| {
            if *x < 0 {
                (f64::NAN, false)
            } else {
                ((*x as f64).sqrt(), true)
            }
        }
    );

    // -- Cube Root --
    impl_unary_op!(
        noparams, noinplace,
        cbrt, cbrt_range,
        f64,
        |x: &i64| (*x as f64).cbrt()
    );

    // -- Nth Root --
    impl_unary_op!(
        params_valid,
        noinplace,
        nth_root, nth_root_range,
        (n: f64) -> (n),
        f64,
        |x: &i64, n: f64| {
            let y = (*x as f64).powf(1.0 / n);
            if y.is_nan() {
                (0_f64, false)
            } else {
                (y, true)
            }
        }
    );

    // -- Reciprocal Square Root --
    impl_unary_op!(
        noparams_valid, noinplace,
        rsqrt, rsqrt_range,
        f64,
        |x: &i64| {
            if *x < 0 {
                (0_f64, false)
            } else {
                let root = (*x as f64).sqrt();
                if root == 0.0 {
                    (f64::INFINITY, true)
                } else {
                    (1.0 / root, true)
                }
            }
        }
    );

    // -- Reciprocal Cube Root --
    impl_unary_op!(
        noparams, noinplace, rcbrt, rcbrt_range,
        f64,
        |x: &i64| {
            let root = (*x as f64).cbrt();
            if root == 0.0 {
                f64::INFINITY
            } else {
                1.0 / root
            }
        }
    );
}