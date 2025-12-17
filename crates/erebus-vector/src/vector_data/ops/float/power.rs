// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<f64> {
    // -- Power Float --
    impl_unary_op!(
        params_valid, inplace,
        powf, powf_inplace, powf_range,
        (power: f64) -> (power),
        f64,
        |x: &f64, power: f64| {
            let y = x.powf(power);
            if y.is_nan() { (0_f64, false) } else { (y, true) }
        },
        |x: &mut f64, power: f64| {
            let y = x.powf(power);
            if y.is_nan() {
                false
            } else {
                *x = y;
                true
            }
        }
    );

    // -- Power Integer --
    impl_unary_op!(
        params_valid, inplace,
        powi, powi_inplace, powi_range,
        (power: i64) -> (power),
        f64,
        |x: &f64, power: i64| {
            let y = x.powi(power as i32);
            if y.is_nan() { (0_f64, false) } else { (y, true) }
        },
        |x: &mut f64, power: i64| {
            let y = x.powi(power as i32);
            if y.is_nan() {
                false
            } else {
                *x = y;
                true
            }
        }
    );

    // -- Power Integer Positive --
    impl_unary_op!(
        params, inplace,
        powi_pos, powi_pos_inplace, powi_pos_range,
        (power: u64) -> (power),
        f64,
        |x: &f64, power: u64| x.powi(power as i32),
        |x: &mut f64, power: u64| *x = x.powi(power as i32)
    );
}