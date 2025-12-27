// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<i64> {

    // -- Less Than --
    impl_numeric_cmp_op!(
        params,
        lt, lt_range,
        (value: i64) -> (value),
        bool,
        |x: &i64| *x < value
    );

    // -- Less Than Equal --
    impl_numeric_cmp_op!(
        params,
        lte, lte_range,
        (value: i64) -> (value),
        bool,
        |x: &i64| *x <= value
    );

    // -- Greater Than --
    impl_numeric_cmp_op!(
        params,
        gt, gt_range,
        (value: i64) -> (value),
        bool,
        |x: &i64| *x > value
    );

    // -- Greater Than Equal --
    impl_numeric_cmp_op!(
        params,
        gte, gte_range,
        (value: i64) -> (value),
        bool,
        |x: &i64| *x >= value
    );

    // -- Threshold --
    impl_numeric_cmp_op!(
        params,
        threshold, threshold_range,
        (value: f64) -> (value),
        bool,
        |x: &i64| (*x as f64) > value
    );

    // -- Threshold Equal --
    impl_numeric_cmp_op!(
        params,
        threshold_eq, threshold_eq_range,
        (value: f64) -> (value),
        bool,
        |x: &i64| (*x as f64) >= value
    );

    // -- Binarize --
    impl_numeric_cmp_op!(
        params,
        binarize, binarize_range,
        (value: f64) -> (value),
        i64,
        |x: &i64| if (*x as f64) > value { 1 } else { 0 }
    );

    // -- Binarize Equal --
    impl_numeric_cmp_op!(
        params,
        binarize_eq, binarize_eq_range,
        (value: f64) -> (value),
        i64,
        |x: &i64| if (*x as f64) >= value { 1 } else { 0 }
    );

    // -- Between (lower ≤ x ≤ upper) -> bool --
    impl_numeric_cmp_op!(
        params_valid,
        between, between_range,
        (lo: i64, hi: i64) -> (lo, hi),
        bool,
        |x: &i64, lo: i64, hi: i64| {
            if lo > hi {
                (false, false)
            } else {
                let v = *x >= lo && *x <= hi;
                (v, true)
            }
        }
    );
}