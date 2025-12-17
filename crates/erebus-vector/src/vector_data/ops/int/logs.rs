// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<i64> {

    // -- Natural Log --
    impl_unary_op!(
        noparams_valid, noinplace,
        ln, ln_range,
        f64,
        |x: &i64| {
            if *x > 0 {
                ((*x as f64).ln(), true)
            } else {
                (0_f64, false)
            }
        }
    );

    // -- Natural Log + 1 --
    impl_unary_op!(
        noparams_valid, noinplace,
        ln_1p, ln_1p_range,
        f64,
        |x: &i64| {
            if *x > -1 {
                ((*x as f64).ln_1p(), true)
            } else {
                (0_f64, false)
            }
        }
    );

    // -- Log Alias for Natural Log --
    #[inline]
    pub fn log(&self) -> VectorData<f64> {
        self.ln()
    }
    #[inline]
    pub fn log_range(&self, start: usize, end: usize, full: bool) -> VectorData<f64> {
        self.ln_range(start, end, full)
    }

    // -- Log 2 --
    impl_unary_op!(
        noparams_valid, noinplace,
        log2, log2_range,
        f64,
        |x: &i64| {
            if *x > 0 {
                ((*x as f64).log2(), true)
            } else {
                (0_f64, false)
            }
        }
    );

    // -- Log 10 --
    impl_unary_op!(
        noparams_valid, noinplace,
        log10, log10_range,
        f64,
        |x: &i64| {
            if *x > 0 {
                ((*x as f64).log10(), true)
            } else {
                (0_f64, false)
            }
        }
    );

    // -- Log(Base) --
    impl_unary_op!(
        params_valid, noinplace,
        logb, logb_range,
        (base: f64) -> (base),
        f64,
        |x: &i64, base: f64| {
            if *x > 0 && base > 0.0 && base != 1.0 {
                ((*x as f64).log(base), true)
            } else {
                (0_f64, false)
            }
        }
    );
}