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
            if y.is_finite() {
                (y, true)
            } else {
                (0.0, false)
            }
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
}