// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<f64> {

    // -- Natural Log --
    impl_unary_op!(
        noparams_valid, inplace,
        ln, ln_inplace, ln_range,
        f64,
        |x: &f64| {
            if *x > 0.0 {
                (x.ln(), true)
            } else {
                (0_f64, false)
            }
        },
        |x: &mut f64| {
            if *x > 0.0 {
                *x = x.ln();
                true
            } else {
                false
            }
        }
    );

    // -- Natural Log + 1 --
    impl_unary_op!(
        noparams_valid, inplace,
        ln_1p, ln_1p_inplace, ln_1p_range,
        f64,
        |x: &f64| {
            if *x > -1.0 {
                (x.ln_1p(), true)
            } else {
                (0_f64, false)
            }
        },
        |x: &mut f64| {
            if *x > -1.0 {
                *x = x.ln_1p();
                true
            } else {
                false
            }
        }
    );

    // -- Log Alias for Natural Log --
    #[inline]
    pub fn log(&self) -> VectorData<f64> {
        self.ln()
    }
    #[inline]
    pub fn log_inplace(&mut self) {
        self.ln_inplace()
    }
    #[inline]
    pub fn log_range(&self, start: usize, end: usize, full: bool) -> VectorData<f64> {
        self.ln_range(start, end, full)
    }

    // -- Log 2 --
    impl_unary_op!(
        noparams_valid, inplace,
        log2, log2_inplace, log2_range,
        f64,
        |x: &f64| {
            if *x > 0.0 {
                (x.log2(), true)
            } else {
                (0_f64, false)
            }
        },
        |x: &mut f64| {
            if *x > 0.0 {
                *x = x.log2();
                true
            } else {
                false
            }
        }
    );

    // -- Log 10 --
    impl_unary_op!(
        noparams_valid, inplace,
        log10, log10_inplace, log10_range,
        f64,
        |x: &f64| {
            if *x > 0.0 {
                (x.log10(), true)
            } else {
                (0_f64, false)
            }
        },
        |x: &mut f64| {
            if *x > 0.0 {
                *x = x.log10();
                true
            } else {
                false
            }
        }
    );

    // -- Log(Base) --
    impl_unary_op!(
        params_valid, inplace,
        logb, logb_inplace, logb_range,
        (base: f64) -> (base),
        f64,
        |x: &f64, base: f64| {
            if *x > 0.0 && base > 0.0 && base != 1.0 {
                (x.log(base), true)
            } else {
                (0_f64, false)
            }
        },
        |x: &mut f64, base: f64| {
            if *x > 0.0 && base > 0.0 && base != 1.0 {
                *x = x.log(base);
                true
            } else {
                false
            }
        }
    );
}