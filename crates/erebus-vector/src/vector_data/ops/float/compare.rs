// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<f64> {

    // -- Less Than --
    impl_numeric_cmp_op!(
        params,
        lt, lt_range,
        (value: f64) -> (value),
        bool,
        |x: &f64| *x < value
    );

    // -- Less Than Equal --
    impl_numeric_cmp_op!(
        params,
        lte, lte_range,
        (value: f64) -> (value),
        bool,
        |x: &f64| *x <= value
    );

    // -- Greater Than --
    impl_numeric_cmp_op!(
        params,
        gt, gt_range,
        (value: f64) -> (value),
        bool,
        |x: &f64| *x > value
    );

    // -- Greater Than Equal --
    impl_numeric_cmp_op!(
        params,
        gte, gte_range,
        (value: f64) -> (value),
        bool,
        |x: &f64| *x >= value
    );

    // -- Threshold --
    impl_numeric_cmp_op!(
        params,
        threshold, threshold_range,
        (value: f64) -> (value),
        bool,
        |x: &f64| *x > value
    );

    // -- Threshold Equal --
    impl_numeric_cmp_op!(
        params,
        threshold_eq, threshold_eq_range,
        (value: f64) -> (value),
        bool,
        |x: &f64| *x >= value
    );

    // -- Binarize --
    impl_numeric_cmp_op!(
        params,
        binarize, binarize_range,
        (value: f64) -> (value),
        i64,
        |x: &f64| if *x > value { 1 } else { 0 }
    );

    // -- Binarize Equal --
    impl_numeric_cmp_op!(
        params,
        binarize_eq, binarize_eq_range,
        (value: f64) -> (value),
        i64,
        |x: &f64| if *x >= value { 1 } else { 0 }
    );

    // -- Between (lower ≤ x ≤ upper) -> bool --
    impl_numeric_cmp_op!(
        params_valid,
        between, between_range,
        (lo: f64, hi: f64) -> (lo, hi),
        bool,
        |x: &f64, lo: f64, hi: f64| {
            if lo > hi {
                (false, false)
            } else {
                let v = *x >= lo && *x <= hi;
                (v, true)
            }
        }
    );

}