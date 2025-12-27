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
            if y.is_finite() { (y, true) } else { (0.0, false) }
        }
    );

    // -- Asin --
    impl_unary_op!(
        noparams_valid, noinplace,
        asin, asin_range,
        f64,
        |x: &i64| {
            let y = (*x as f64).asin();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        }
    );

    // -- Acos --
    impl_unary_op!(
        noparams_valid, noinplace,
        acos, acos_range,
        f64,
        |x: &i64| {
            let y = (*x as f64).acos();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        }
    );

    // -- Atan --
    impl_unary_op!(
        noparams, noinplace,
        atan, atan_range,
        f64,
        |x: &i64| (*x as f64).atan()
    );

    // -- Sinh --
    impl_unary_op!(
        noparams_valid, noinplace,
        sinh, sinh_range,
        f64,
        |x: &i64| {
            let y = (*x as f64).sinh();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        }
    );

    // -- Cosh --
    impl_unary_op!(
        noparams_valid, noinplace,
        cosh, cosh_range,
        f64,
        |x: &i64| {
            let y = (*x as f64).cosh();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        }
    );

    // -- Tanh --
    impl_unary_op!(
        noparams, noinplace,
        tanh, tanh_range,
        f64,
        |x: &i64| (*x as f64).tanh()
    );

    // -- Asinh --
    impl_unary_op!(
        noparams, noinplace,
        asinh, asinh_range,
        f64,
        |x: &i64| (*x as f64).asinh()
    );

    // -- Acosh --
    impl_unary_op!(
        noparams_valid, noinplace,
        acosh, acosh_range,
        f64,
        |x: &i64| {
            let y = (*x as f64).acosh();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        }
    );

    // -- Atanh --
    impl_unary_op!(
        noparams_valid, noinplace,
        atanh, atanh_range,
        f64,
        |x: &i64| {
            let y = (*x as f64).atanh();
            if y.is_finite() { (y, true) } else { (0.0, false) }
        }
    );
}