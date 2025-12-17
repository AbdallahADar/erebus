// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<f64> {
    // -- Square Root --
    impl_unary_op!(
        noparams_valid, inplace,
        sqrt, sqrt_inplace, sqrt_range,
        f64,
        |x: &f64| {
            if *x < 0.0 {
                (f64::NAN, false)
            } else {
                (x.sqrt(), true)
            }
        },
        |x: &mut f64| {
            if *x < 0.0 {
                // mark invalid
                false
            } else {
                *x = x.sqrt();
                true
            }
        }
    );

    // -- Cube Root --
    impl_unary_op!(
        noparams, inplace,
        cbrt, cbrt_inplace, cbrt_range,
        f64,
        |x: &f64| x.cbrt(),
        |x: &mut f64| { *x = x.cbrt(); }
    );

    // -- Nth Root --
    impl_unary_op!(
        params_valid,
        inplace,
        nth_root, nth_root_inplace, nth_root_range,
        (n: f64) -> (n),
        f64,
        |x: &f64, n: f64| {
            let y = x.powf(1.0 / n);
            if y.is_nan() {
                (0_f64, false)
            } else {
                (y, true)
            }
        },
        |x: &mut f64, n: f64| {
            let y = (*x).powf(1.0 / n);
            if y.is_nan() {
                false
            } else {
                *x = y;
                true
            }
        }
    );

    // -- Reciprocal Square Root --
    impl_unary_op!(
        noparams_valid, inplace,
        rsqrt, rsqrt_inplace, rsqrt_range,
        f64,
        |x: &f64| {
            if *x < 0.0 {
                (0_f64, false)
            } else {
                let root = x.sqrt();
                if root == 0.0 {
                    (f64::INFINITY, true)
                } else {
                    (1.0 / root, true)
                }
            }
        },
        |x: &mut f64| {
            if *x < 0.0 {
                false
            } else {
                let root = x.sqrt();
                *x = if root == 0.0 { f64::INFINITY } else { 1.0 / root };
                true
            }
        }
    );


    // -- Reciprocal Cube Root --
    impl_unary_op!(
        noparams, inplace,
        rcbrt, rcbrt_inplace, rcbrt_range,
        f64,
        |x: &f64| {
            let root = x.cbrt();
            if root == 0.0 {
                f64::INFINITY
            } else {
                1.0 / root
            }
        },
        |x: &mut f64| {
            let root = x.cbrt();
            *x = if root == 0.0 { f64::INFINITY } else { 1.0 / root };
        }
    );
}