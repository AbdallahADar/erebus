// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<f64> {

    // -- Sin --
    impl_unary_op!(
        noparams, inplace,
        sin, sin_inplace, sin_range,
        f64,
        |x: &f64| x.sin(),
        |x: &mut f64| { *x = x.sin(); }
    );

    // -- Cos --
    impl_unary_op!(
        noparams, inplace,
        cos, cos_inplace, cos_range,
        f64,
        |x: &f64| x.cos(),
        |x: &mut f64| { *x = x.cos(); }
    );

    // -- Tan --
    impl_unary_op!(
        noparams_valid, inplace,
        tan, tan_inplace, tan_range,
        f64,
        |x: &f64| {
            let y = x.tan();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        },
        |x: &mut f64| {
            let y = x.tan();
            if y.is_finite() {
                *x = y;
                true
            } else {
                *x = 0.0;
                false
            }
        }
    );

    // -- Asin --
    impl_unary_op!(
        noparams_valid, inplace,
        asin, asin_inplace, asin_range,
        f64,
        |x: &f64| {
            let y = x.asin();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        },
        |x: &mut f64| {
            let y = x.asin();
            if y.is_finite() {
                *x = y;
                true
            } else {
                false
            }
        }
    );

    // -- Acos --
    impl_unary_op!(
        noparams_valid, inplace,
        acos, acos_inplace, acos_range,
        f64,
        |x: &f64| {
            let y = x.acos();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        },
        |x: &mut f64| {
            let y = x.acos();
            if y.is_finite() {
                *x = y;
                true
            } else {
                false
            }
        }
    );

    // -- Atan --
    impl_unary_op!(
        noparams, inplace,
        atan, atan_inplace, atan_range,
        f64,
        |x: &f64| x.atan(),
        |x: &mut f64| *x = x.atan()
    );

    // -- Sinh --
    impl_unary_op!(
        noparams_valid, inplace,
        sinh, sinh_inplace, sinh_range,
        f64,
        |x: &f64| {
            let y = x.sinh();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        },
        |x: &mut f64| {
            let y = x.sinh();
            if y.is_finite() {
                *x = y;
                true
            } else {
                false
            }
        }
    );

    // -- Cosh --
    impl_unary_op!(
        noparams_valid, inplace,
        cosh, cosh_inplace, cosh_range,
        f64,
        |x: &f64| {
            let y = x.cosh();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        },
        |x: &mut f64| {
            let y = x.cosh();
            if y.is_finite() {
                *x = y;
                true
            } else {
                false
            }
        }
    );

    // -- Tanh --
    impl_unary_op!(
        noparams, inplace,
        tanh, tanh_inplace, tanh_range,
        f64,
        |x: &f64| x.tanh(),
        |x: &mut f64| *x = x.tanh()
    );

    // -- Asinh --
    impl_unary_op!(
        noparams, inplace,
        asinh, asinh_inplace, asinh_range,
        f64,
        |x: &f64| x.asinh(),
        |x: &mut f64| *x = x.asinh()
    );

    // -- Acosh --
    impl_unary_op!(
        noparams_valid, inplace,
        acosh, acosh_inplace, acosh_range,
        f64,
        |x: &f64| {
            let y = x.acosh();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        },
        |x: &mut f64| {
            let y = x.acosh();
            if y.is_finite() {
                *x = y;
                true
            } else {
                false
            }
        }
    );

    // -- Atanh --
    impl_unary_op!(
        noparams_valid, inplace,
        atanh, atanh_inplace, atanh_range,
        f64,
        |x: &f64| {
            let y = x.atanh();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        },
        |x: &mut f64| {
            let y = x.atanh();
            if y.is_finite() {
                *x = y;
                true
            } else {
                false
            }
        }
    );
}