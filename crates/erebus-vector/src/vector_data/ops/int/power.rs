// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<i64> {
    // -- Power Float --
    impl_unary_op!(
        params_valid, noinplace,
        powf, powf_range,
        (power: f64) -> (power),
        f64,
        |x: &i64, power: f64| {
            let y = (*x as f64).powf(power);
            if y.is_nan() { (0_f64, false) } else { (y, true) }
        }
    );

    // -- Power Integer --
    impl_unary_op!(
        params_valid, noinplace,
        powi, powi_range,
        (power: i64) -> (power),
        f64,
        |x: &i64, power: i64| {
            let y = (*x as f64).powi(power as i32);
            if y.is_nan() { (0_f64, false) } else { (y, true) }
        }
    );

    // -- Power Integer Positive --
    impl_unary_op!(
        params, inplace,
        powi_pos, powi_pos_inplace, powi_pos_range,
        (power: u64) -> (power),
        i64,
        |x: &i64, power: u64| x.pow(power as u32),
        |x: &mut i64, power: u64| *x = x.pow(power as u32)
    );
}