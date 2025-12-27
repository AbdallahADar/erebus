// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

fn vd_i64(data: Vec<i64>, valid: Vec<bool>) -> VectorData<i64> {
    VectorData::from_vec(data, valid.into_iter().collect()).unwrap()
}

fn vd_f64(data: Vec<f64>, valid: Vec<bool>) -> VectorData<f64> {
    VectorData::from_vec(data, valid.into_iter().collect()).unwrap()
}

fn vd_bool(v: Vec<bool>, valid: Vec<bool>) -> VectorData<bool> {
    VectorData::from_vec(v, BitVec::from_iter(valid)).unwrap()
}

fn assert_f64_eq(a: f64, b: f64) {
    if a.is_nan() && b.is_nan() {
        return;
    }
    if a.is_infinite() || b.is_infinite() {
        assert!(a == b, "expected {b}, got {a}");
        return;
    }
    assert!(
        (a - b).abs() < 1e-12,
        "expected {b}, got {a}"
    );
}

fn assert_vec_approx(a: &[f64], b: &[f64]) {
    assert_eq!(a.len(), b.len());
    for (x, y) in a.iter().zip(b.iter()) {
        assert_f64_eq(*x, *y);
    }
}

fn assert_vd_f64_matches<F>(
    out: &VectorData<f64>,
    input: &[f64],
    validity: &[bool],
    f: F,
)
where
    F: Fn(f64) -> f64,
{
    assert_eq!(out.data.len(), input.len());
    assert_eq!(out.validity.len(), input.len());

    for i in 0..input.len() {
        // Original NA always stays NA
        if !validity[i] {
            assert!(!out.validity[i]);
            continue;
        }

        let expected = f(input[i]);

        if !expected.is_finite() {
            assert!(
                !out.validity[i],
                "expected NA at index {i}, got finite validity with value {}",
                out.data[i]
            );
        } else {
            assert!(
                out.validity[i],
                "expected valid at index {i}, got NA"
            );
            assert_f64_eq(out.data[i], expected);
        }
    }
}

fn assert_vd_f64_matches_range<F>(
    out: &VectorData<f64>,
    input: &[f64],
    validity: &[bool],
    start: usize,
    end: usize,
    f: F,
)
where
    F: Fn(f64) -> f64,
{
    for i in 0..input.len() {
        if !validity[i] {
            assert!(!out.validity[i]);
            continue;
        }

        if i >= start && i < end {
            let expected = f(input[i]);
            if expected.is_finite() {
                assert!(out.validity[i]);
                assert_f64_eq(out.data[i], expected);
            } else {
                assert!(!out.validity[i]);
            }
        } else {
            assert!(out.validity[i]);
            assert_f64_eq(out.data[i], input[i]);
        }
    }
}

#[test]
fn test_abs_i64_nulls_basic() {
    // data: [-5, 3, -1, 0]
    // valid: [ 0, 1,  0, 1 ]
    let v = vd_i64(vec![-5, 3, -1, 0], vec![false, true, false, true]);

    let out = v.abs();

    assert_eq!(out.data, vec![5, 3, 1, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1, 0, 1]);
}

#[test]
fn test_abs_i64_nulls_inplace() {
    let mut v = vd_i64(vec![-5, 3, -1, 0], vec![false, true, false, true]);

    v.abs_inplace();

    assert_eq!(v.data, vec![5, 3, 1, 0]);
    assert_eq!(v.validity.to_vec(), bitvec![0, 1, 0, 1]); // unchanged
}

#[test]
fn test_abs_i64_nulls_range_slice_only() {
    // Slice [1,3)
    let v = vd_i64(vec![-5, 3, -1, 0], vec![false, true, false, true]);

    let out = v.abs_range(1, 3, false);

    assert_eq!(out.data, vec![3, 1]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0]);
}

#[test]
fn test_abs_i64_nulls_range_full() {
    // original:
    //  data: [-5, 3, -1, 0]
    // valid: [ 0, 1,  0, 1 ]
    //
    // apply abs to indices 1..3 → [3,1]
    //
    // expected full reconstruction:
    //  data: [-5, 3, 1, 0]
    // valid: [ 0, 1, 0, 1 ]
    let v = vd_i64(vec![-5, 3, -1, 0], vec![false, true, false, true]);

    let out = v.abs_range(1, 3, true);

    assert_eq!(out.data, vec![-5, 3, 1, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1, 0, 1]);
}

#[test]
fn test_abs_f64_nulls_basic() {
    let v = vd_f64(vec![-2.5, 3.0, -0.4, 0.0], vec![true, false, true, false]);

    let out = v.abs();

    assert_eq!(out.data, vec![2.5, 3.0, 0.4, 0.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1, 0]);
}

#[test]
fn test_abs_f64_nulls_inplace() {
    let mut v = vd_f64(vec![-2.5, 3.0, -0.4, 0.0], vec![true, false, true, false]);

    v.abs_inplace();

    assert_eq!(v.data, vec![2.5, 3.0, 0.4, 0.0]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 0, 1, 0]);
}

#[test]
fn test_abs_f64_nulls_range_slice_only() {
    let v = vd_f64(vec![-2.5, 3.0, -0.4, 0.0], vec![true, false, true, false]);

    let out = v.abs_range(1, 3, false);

    assert_eq!(out.data, vec![3.0, 0.4]);       // transformed only
    assert_eq!(out.validity.to_vec(), bitvec![0, 1]); // slice validity preserved
}

#[test]
fn test_abs_f64_nulls_range_full() {
    let v = vd_f64(vec![-2.5, 3.0, -0.4, 0.0], vec![true, false, true, false]);

    // Expected:
    // data: [-2.5, 3.0, 0.4, 0.0]
    // valid: [1, 0, 1, 0]
    let out = v.abs_range(1, 3, true);

    assert_eq!(out.data, vec![-2.5, 3.0, 0.4, 0.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1, 0]);
}

#[test]
fn test_exp_i64_owned() {
    let v = vd_i64(vec![0, 1, -1], vec![true, true, true]);

    let out = v.exp();

    let expected = vec![
        (0.0_f64).exp(),
        (1.0_f64).exp(),
        (-1.0_f64).exp(),
    ];

    assert_eq!(out.data, expected);
    assert_eq!(out.validity.to_vec(), bitvec![1,1,1]);
}

#[test]
fn test_exp_i64_with_nulls() {
    let v = vd_i64(vec![0, 1, -1], vec![true, false, true]);

    let out = v.exp();

    let expected = vec![
        (0.0_f64).exp(),
        (1.0_f64).exp(),   // value computed but validity = 0
        (-1.0_f64).exp(),
    ];

    assert_eq!(out.data, expected);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_exp_i64_range() {
    let v = vd_i64(vec![10, 20, 30, 40], vec![true; 4]);

    // only transform [1,3), but return full = true
    let out = v.exp_range(1, 3, true);

    let expected = vec![
        10.0_f64,              // unchanged (converted)
        (20.0_f64).exp(),      // transformed
        (30.0_f64).exp(),      // transformed
        40.0_f64,              // unchanged (converted)
    ];

    assert_eq!(out.data, expected);
}

#[test]
fn test_exp_f64_owned() {
    let v = vd_f64(vec![0.0, 1.0, -1.0], vec![true; 3]);

    let out = v.exp();

    assert_eq!(out.data, vec![
        0.0_f64.exp(),
        1.0_f64.exp(),
        (-1.0_f64).exp()
    ]);
}

#[test]
fn test_exp_f64_inplace() {
    let mut v = vd_f64(vec![0.0, 2.0], vec![true; 2]);

    v.exp_inplace();

    assert_eq!(v.data, vec![
        0.0_f64.exp(),
        2.0_f64.exp()
    ]);
}

#[test]
fn test_exp_f64_with_nulls() {
    let mut v = vd_f64(vec![0.0, 2.0, -1.0], vec![true, false, true]);

    let out = v.exp();

    assert_eq!(out.data, vec![
        0.0_f64.exp(),
        2.0_f64.exp(),
        (-1.0_f64).exp()
    ]);

    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_exp_f64_range() {
    let v = vd_f64(vec![1.0, 2.0, 3.0], vec![true; 3]);

    let out = v.exp_range(1, 3, true);

    assert_eq!(out.data, vec![
        1.0,               // unchanged
        2.0_f64.exp(),     // transformed
        3.0_f64.exp(),     // transformed
    ]);
}

#[test]
fn test_exp_m1_i64() {
    let v = vd_i64(vec![0, 1, -1], vec![true; 3]);

    let out = v.exp_m1();

    assert_eq!(out.data, vec![
        0.0_f64.exp_m1(),
        1.0_f64.exp_m1(),
        (-1.0_f64).exp_m1()
    ]);
}

#[test]
fn test_exp_m1_f64() {
    let v = vd_f64(vec![0.0, 0.5, -0.5], vec![true; 3]);

    let out = v.exp_m1();

    assert_eq!(out.data, vec![
        0.0_f64.exp_m1(),
        0.5_f64.exp_m1(),
        (-0.5_f64).exp_m1()
    ]);
}

#[test]
fn test_exp_m1_f64_inplace() {
    let mut v = vd_f64(vec![0.0, 1.0], vec![true; 2]);

    v.exp_m1_inplace();

    assert_eq!(v.data, vec![
        0.0_f64.exp_m1(),
        1.0_f64.exp_m1(),
    ]);
}

#[test]
fn test_exp_m1_f64_range() {
    let v = vd_f64(vec![10.0, 20.0, 30.0], vec![true; 3]);

    let out = v.exp_m1_range(1, 3, true);

    assert_eq!(out.data, vec![
        10.0,
        20.0_f64.exp_m1(),
        30.0_f64.exp_m1(),
    ]);
}


#[test]
fn test_int_round_owned() {
    let v = vd_i64(vec![-3, 2, 0, 5], vec![true, false, true, true]);

    assert_eq!(v.ceil().data,  vec![-3,2,0,5]);
    assert_eq!(v.floor().data, vec![-3,2,0,5]);
    assert_eq!(v.round().data, vec![-3,2,0,5]);

    assert_eq!(v.trunc().data, vec![-3,2,0,5]);
    assert_eq!(v.fract().data, vec![0,0,0,0]);

    assert_eq!(v.signum().data, vec![-1,1,0,1]);
    assert_eq!(v.validity.to_vec(), bitvec![1,0,1,1]);
}

#[test]
fn test_int_round_inplace() {
    let mut v = vd_i64(vec![-3, 2, 0, 5], vec![true, false, true, true]);

    v.signum_inplace();

    assert_eq!(v.data, vec![-1, 1, 0, 1]);
    assert_eq!(v.validity.to_vec(), bitvec![1,0,1,1]);
}

#[test]
fn test_float_round_owned() {
    let v = vd_f64(vec![-2.7, 3.2, 0.0], vec![true,false,true]);

    assert_eq!(v.ceil().data,  vec![-2.0, 4.0, 0.0]);
    assert_eq!(v.floor().data, vec![-3.0, 3.0, 0.0]);
    assert_eq!(v.round().data, vec![-3.0, 3.0, 0.0]);

    assert_eq!(v.trunc().data, vec![-2.0, 3.0, 0.0]);
    assert_vec_approx(&v.fract().data, &[-0.7, 0.2, 0.0]); // Approx used due to floating point error

    assert_eq!(v.roundup().data, vec![-2.0, 4.0, 0.0]);

    assert_eq!(v.signum().data, vec![-1.0, 1.0, 0.0]);
}

#[test]
fn test_i64_floor_range_slice_only() {
    let v = vd_i64(vec![-3, 2, 7, -1], vec![true, false, true, true]);

    // Apply floor to indices [1,3) → [2, 7]
    let out = v.floor_range(1, 3, false);

    assert_eq!(out.data, vec![2, 7]);            // slice transformed (though floor(x)=x)
    assert_eq!(out.validity.to_vec(), bitvec![0, 1]);
}

#[test]
fn test_i64_floor_range_full() {
    let v = vd_i64(vec![-3, 2, 7, -1], vec![true, false, true, true]);

    // full reconstruction
    let out = v.floor_range(1, 3, true);

    assert_eq!(out.data, vec![-3, 2, 7, -1]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1, 1]);
}

#[test]
fn test_i64_round_range_full() {
    let v = vd_i64(vec![-3, 2, 7, -1], vec![true, true, false, true]);

    let out = v.round_range(0, 2, true);

    assert_eq!(out.data, vec![-3, 2, 7, -1]); // unchanged for ints
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 0, 1]);
}

#[test]
fn test_i64_trunc_range_slice_only() {
    let v = vd_i64(vec![5, -4, 9], vec![true, false, true]);

    let out = v.trunc_range(1, 3, false);

    assert_eq!(out.data, vec![-4, 9]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1]);
}

#[test]
fn test_i64_fract_range_slice_only() {
    let v = vd_i64(vec![10, -3, 7], vec![true, true, false]);

    let out = v.fract_range(0, 2, false);

    assert_eq!(out.data, vec![0, 0]);  // always zero for ints
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_i64_signum_range_full() {
    let v = vd_i64(vec![-4, 0, 9, -2], vec![true, false, true, true]);

    let out = v.signum_range(1, 4, true);

    assert_eq!(out.data, vec![-4, 0, 1, -1]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1, 1]);
}

#[test]
fn test_f64_floor_range_slice_only() {
    let v = vd_f64(vec![-2.7, 3.2, 5.9, 1.0], vec![true, false, true, true]);

    let out = v.floor_range(1, 3, false);

    assert_eq!(out.data, vec![3.0, 5.0]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1]);
}

#[test]
fn test_f64_floor_range_full() {
    let v = vd_f64(vec![-2.7, 3.2, 5.9, 1.0], vec![true, false, true, true]);

    let out = v.floor_range(1, 3, true);

    assert_eq!(out.data, vec![-2.7, 3.0, 5.0, 1.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1, 1]);
}

#[test]
fn test_f64_round_range_slice_only() {
    let v = vd_f64(vec![-2.7, 3.2, 5.9], vec![true, true, false]);

    let out = v.round_range(0, 2, false);

    assert_eq!(out.data, vec![-3.0, 3.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_f64_round_range_full() {
    let v = vd_f64(vec![-2.7, 3.2, 5.9], vec![true, true, false]);

    let out = v.round_range(0, 2, true);

    assert_eq!(out.data, vec![-3.0, 3.0, 5.9]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 0]);
}

#[test]
fn test_f64_fract_range_slice_only() {
    let v = vd_f64(vec![1.2, 3.75, -2.5], vec![true, false, true]);

    let out = v.fract_range(1, 3, false);

    assert_eq!(out.data, vec![0.75, -0.5]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1]);
}

#[test]
fn test_f64_trunc_range_full() {
    let v = vd_f64(vec![1.2, 3.75, -2.5], vec![true, false, true]);

    let out = v.trunc_range(0, 2, true);

    assert_eq!(out.data, vec![1.0, 3.0, -2.5]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_f64_signum_range_full() {
    let v = vd_f64(vec![-2.7, 0.0, 5.4, -1.1], vec![true, false, true, true]);

    let out = v.signum_range(1, 4, true);

    assert_eq!(out.data, vec![-2.7, 0.0, 1.0, -1.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1, 1]);
}

#[test]
fn test_f64_roundup_range_slice_only() {
    let v = vd_f64(vec![-2.7, 3.2, 5.1], vec![true, true, false]);

    let out = v.roundup_range(1, 3, false);

    assert_eq!(out.data, vec![4.0, 6.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0]);
}

#[test]
fn test_i64_neg_owned_basic() {
    let v = vd_i64(vec![1, -2, 0], vec![true, true, true]);

    let out = v.neg();

    assert_eq!(out.data, vec![-1, 2, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_i64_neg_owned_with_nulls() {
    let v = vd_i64(vec![1, -2, 3], vec![true, false, true]);

    let out = v.neg();

    assert_eq!(out.data, vec![-1, 2, -3]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_i64_neg_inplace_basic() {
    let mut v = vd_i64(vec![1, -2, 0], vec![true, true, true]);

    v.neg_inplace();

    assert_eq!(v.data, vec![-1, 2, 0]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_i64_neg_inplace_with_nulls() {
    let mut v = vd_i64(vec![1, -2, 3], vec![true, false, true]);

    v.neg_inplace();

    assert_eq!(v.data, vec![-1, 2, -3]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 0, 1]);
}

// range: slice-only
#[test]
fn test_i64_neg_range_slice_only() {
    let v = vd_i64(vec![10, -5, 3, -1], vec![true, true, false, true]);

    let out = v.neg_range(1, 3, false);

    // indices 1..3 → [-5, 3] → [5, -3]
    assert_eq!(out.data, vec![5, -3]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0]); // validity slice copied
}

// range: full reconstruction
#[test]
fn test_i64_neg_range_full() {
    let v = vd_i64(vec![10, -5, 3, -1], vec![true, true, false, true]);

    let out = v.neg_range(1, 3, true);

    // original:  [10, -5, 3, -1]
    // neg on 1..3 => [10, 5, -3, -1]
    assert_eq!(out.data, vec![10, 5, -3, -1]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 0, 1]);
}

// flip_sign: same as neg, just alias

#[test]
fn test_i64_flip_sign_owned_basic() {
    let v = vd_i64(vec![1, -2, 3], vec![true, true, true]);

    let out = v.flip_sign();

    assert_eq!(out.data, vec![-1, 2, -3]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_i64_flip_sign_inplace() {
    let mut v = vd_i64(vec![1, -2, 3], vec![true, false, true]);

    v.flip_sign_inplace();

    assert_eq!(v.data, vec![-1, 2, -3]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_i64_flip_sign_range_full() {
    let v = vd_i64(vec![1, -2, 3, -4], vec![true, true, true, true]);

    let out = v.flip_sign_range(0, 2, true);

    // flip signs for [1, -2] => [-1, 2]
    assert_eq!(out.data, vec![-1, 2, 3, -4]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_i64_signbit_owned_basic() {
    let v = vd_i64(vec![-3, 0, 5, -1], vec![true, true, true, false]);

    let out = v.signbit();

    assert_eq!(out.data, vec![true, false, false, true]);
    // validity preserved
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 0]);
}

// slice-only
#[test]
fn test_i64_signbit_range_slice_only() {
    let v = vd_i64(vec![-3, 0, 5], vec![true, false, true]);

    let out = v.signbit_range(0, 2, false);

    // [-3, 0] -> [true, false]
    assert_eq!(out.data, vec![true, false]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0]);
}

// full reconstruction
#[test]
fn test_i64_signbit_range_full() {
    let v = vd_i64(vec![-3, 0, 5], vec![true, false, true]);

    let out = v.signbit_range(1, 3, true);

    // original data: [-3, 0, 5]
    // signbit on indices 1..3 => [signbit(0), signbit(5)] = [false, false]
    // result data: [true, false, false]
    assert_eq!(out.data, vec![true, false, false]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_f64_neg_owned_basic() {
    let v = vd_f64(vec![1.5, -2.0, 0.0], vec![true, true, true]);

    let out = v.neg();

    assert_eq!(out.data, vec![-1.5, 2.0, 0.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_f64_neg_owned_with_nulls() {
    let v = vd_f64(vec![1.5, -2.0, 3.3], vec![true, false, true]);

    let out = v.neg();

    assert_eq!(out.data, vec![-1.5, 2.0, -3.3]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_f64_neg_inplace_basic() {
    let mut v = vd_f64(vec![1.5, -2.0], vec![true, true]);

    v.neg_inplace();

    assert_eq!(v.data, vec![-1.5, 2.0]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_f64_neg_inplace_with_nulls() {
    let mut v = vd_f64(vec![1.5, -2.0, 3.3], vec![true, false, true]);

    v.neg_inplace();

    assert_eq!(v.data, vec![-1.5, 2.0, -3.3]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 0, 1]);
}

// range slice-only
#[test]
fn test_f64_neg_range_slice_only() {
    let v = vd_f64(vec![10.0, -5.5, 3.2, -0.1], vec![true, true, false, true]);

    let out = v.neg_range(1, 4, false);

    assert_eq!(out.data, vec![5.5, -3.2, 0.1]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

// range full
#[test]
fn test_f64_neg_range_full() {
    let v = vd_f64(vec![10.0, -5.5, 3.2, -0.1], vec![true, true, false, true]);

    let out = v.neg_range(1, 4, true);

    assert_eq!(out.data, vec![10.0, 5.5, -3.2, 0.1]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 0, 1]);
}

// flip_sign

#[test]
fn test_f64_flip_sign_owned_basic() {
    let v = vd_f64(vec![1.0, -2.0, 3.5], vec![true, true, true]);

    let out = v.flip_sign();

    assert_eq!(out.data, vec![-1.0, 2.0, -3.5]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_f64_flip_sign_inplace() {
    let mut v = vd_f64(vec![1.0, -2.0, 3.5], vec![true, false, true]);

    v.flip_sign_inplace();

    assert_eq!(v.data, vec![-1.0, 2.0, -3.5]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_f64_flip_sign_range_full() {
    let v = vd_f64(vec![1.0, -2.0, 3.5, -4.5], vec![true, true, true, true]);

    let out = v.flip_sign_range(0, 3, true);

    assert_eq!(out.data, vec![-1.0, 2.0, -3.5, -4.5]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_f64_signbit_owned_basic() {
    let v = vd_f64(vec![-0.0, 0.0, -2.5, 3.3], vec![true, true, true, false]);

    let out = v.signbit();

    assert_eq!(out.data, vec![true, false, true, false]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 0]);
}

#[test]
fn test_f64_signbit_range_slice_only() {
    let v = vd_f64(vec![-0.0, 1.0, -3.0], vec![true, false, true]);

    let out = v.signbit_range(0, 2, false);

    assert_eq!(out.data, vec![true, false]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0]);
}

#[test]
fn test_f64_signbit_range_full() {
    let v = vd_f64(vec![-0.0, 1.0, -3.0], vec![true, false, true]);

    let out = v.signbit_range(1, 3, true);

    // signbit(1.0)=false, signbit(-3.0)=true
    assert_eq!(out.data, vec![false, false, true]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_bool_not_owned_basic() {
    let v = vd_bool(vec![true, false, true], vec![true, true, true]);

    let out = v.not();

    assert_eq!(out.data, vec![false, true, false]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_bool_not_owned_with_nulls() {
    let v = vd_bool(vec![true, false, true], vec![true, false, true]);

    let out = v.not();

    // values flipped where valid, but validity preserved
    assert_eq!(out.data, vec![false, true, false]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_bool_not_inplace_basic() {
    let mut v = vd_bool(vec![true, false], vec![true, true]);

    v.not_inplace();

    assert_eq!(v.data, vec![false, true]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_bool_not_inplace_with_nulls() {
    let mut v = vd_bool(vec![true, false, true], vec![false, true, true]);

    v.not_inplace();

    assert_eq!(v.data, vec![false, true, false]);
    assert_eq!(v.validity.to_vec(), bitvec![0, 1, 1]);
}

// range: slice-only
#[test]
fn test_bool_not_range_slice_only() {
    let v = vd_bool(vec![true, false, true, false], vec![true, true, false, true]);

    let out = v.not_range(1, 4, false);

    // slice [false, true, false] → [true, false, true]
    assert_eq!(out.data, vec![true, false, true]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

// range: full reconstruction
#[test]
fn test_bool_not_range_full() {
    let v = vd_bool(vec![true, false, true, false], vec![true, true, false, true]);

    let out = v.not_range(1, 3, true);

    // original data: [true, false, true, false]
    // not on 1..3 => [true, true, true, false]
    assert_eq!(out.data, vec![true, true, false, false]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 0, 1]);
}

#[test]
fn test_i64_neg_operator_owned() {
    let v = vd_i64(vec![5, -3, 0], vec![true, false, true]);

    let out = -v;   // owned negation

    assert_eq!(out.data, vec![-5, 3, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_i64_neg_operator_ref() {
    let v = vd_i64(vec![10, -2, 7], vec![true, true, false]);

    let out = -&v;  // borrowed negation

    assert_eq!(out.data, vec![-10, 2, -7]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 0]);

    // Ensure original was not mutated
    assert_eq!(v.data, vec![10, -2, 7]);
}

#[test]
fn test_i64_neg_operator_all_nulls() {
    let v = vd_i64(vec![1, 2, 3], vec![false, false, false]);

    let out = -v;

    assert_eq!(out.data, vec![-1, -2, -3]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 0, 0]);
}

#[test]
fn test_f64_neg_operator_owned() {
    let v = vd_f64(vec![2.5, -1.0, 0.0], vec![true, false, true]);

    let out = -v;

    assert_eq!(out.data, vec![-2.5, 1.0, 0.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_f64_neg_operator_ref() {
    let v = vd_f64(vec![-0.5, 3.1, -7.2], vec![true, true, true]);

    let out = -&v;

    assert_eq!(out.data, vec![0.5, -3.1, 7.2]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);

    // Confirm not mutated
    assert_eq!(v.data, vec![-0.5, 3.1, -7.2]);
}

#[test]
fn test_f64_neg_operator_with_nulls() {
    let v = vd_f64(vec![1.5, -4.0, 2.0], vec![false, true, false]);

    let out = -v;

    assert_eq!(out.data, vec![-1.5, 4.0, -2.0]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1, 0]);
}

#[test]
fn test_f64_neg_operator_empty() {
    let v = vd_f64(vec![], vec![]);

    let out = -v;

    assert!(out.data.is_empty());
    assert!(out.validity.is_empty());
}

#[test]
fn test_f64_neg_operator_negative_zero() {
    let v = vd_f64(vec![-0.0, 0.0], vec![true, true]);

    let out = -v;

    // -(-0.0) = +0.0
    // -(+0.0) = -0.0
    assert_eq!(out.data[0].to_bits(), 0.0_f64.to_bits());       // +0.0
    assert_eq!(out.data[1].to_bits(), (-0.0_f64).to_bits());    // -0.0
}

#[test]
fn test_i64_sqrt_owned_basic() {
    let v = vd_i64(vec![9, 0, -4], vec![true, true, true]);

    let out = v.sqrt();

    // values (NaN for negative, but invalid anyway)
    assert_eq!(out.data[0], 3.0);
    assert_eq!(out.data[1], 0.0);
    assert!(out.data[2].is_nan());

    // validity: negative -> false
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 0]);
}

#[test]
fn test_i64_sqrt_range_slice_only() {
    // data: [16, -1, 25]
    // valid: [1,  1,  1]
    let v = vd_i64(vec![16, -1, 25], vec![true, true, true]);

    // slice [1,3), full = false
    let out = v.sqrt_range(1, 3, false);

    // expect: sqrt(-1) -> NaN (invalid), sqrt(25) -> 5
    assert!(out.data[0].is_nan());
    assert_eq!(out.data[1], 5.0);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1]);
}

#[test]
fn test_i64_sqrt_range_full() {
    let v = vd_i64(vec![16, -1, 25], vec![true, true, true]);

    // apply sqrt to [1,3), full = true
    let out = v.sqrt_range(1, 3, true);

    // index 0 unchanged (converted to f64), indices 1..2 transformed
    assert_eq!(out.data[0], 16.0);
    assert!(out.data[1].is_nan());
    assert_eq!(out.data[2], 5.0);

    // validity: index 1 becomes invalid
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_i64_cbrt_owned() {
    let v = vd_i64(vec![27, -8, 1], vec![true, false, true]);

    let out = v.cbrt();

    assert_eq!(out.data, vec![3.0, -2.0, 1.0]);
    // validity unchanged
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_i64_cbrt_range_full() {
    let v = vd_i64(vec![8, 0, -27, 64], vec![true, true, true, true]);

    let out = v.cbrt_range(1, 4, true);

    assert_eq!(out.data, vec![8.0, 0.0, -3.0, 4.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_f64_sqrt_owned_basic() {
    // data: [4.0, 0.0, -9.0]
    // valid: [1,   0,   1   ]
    let v = vd_f64(vec![4.0, 0.0, -9.0], vec![true, false, true]);

    let out = v.sqrt();

    assert_eq!(out.data[0], 2.0);
    assert_eq!(out.data[1], 0.0);        // unchanged value, validity = 0
    assert!(out.data[2].is_nan());       // NaN for negative
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 0]); // index 2 invalidated
}

#[test]
fn test_f64_sqrt_inplace_basic() {
    let mut v = vd_f64(vec![1.0, -4.0, 9.0], vec![true, true, true]);

    v.sqrt_inplace();

    assert_eq!(v.data[0], 1.0);
    assert!(!v.validity[1]);
    assert_eq!(v.data[2], 3.0);

    // validity: index 1 becomes invalid
    assert_eq!(v.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_f64_sqrt_range_full() {
    // data: [1.0, -4.0, 9.0]
    // valid: [1,    1,   0 ]
    let v = vd_f64(vec![1.0, -4.0, 9.0], vec![true, true, false]);

    let out = v.sqrt_range(1, 3, true);

    assert_eq!(out.data[0], 1.0);       // untouched
    assert!(!out.validity[1]);   // negative sqrt
    assert_eq!(out.data[2], 3.0);

    // validity: index 1 invalidated, index 2 was already 0 => stays 0
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 0]);
}

#[test]
fn test_f64_cbrt_owned() {
    let v = vd_f64(vec![8.0, -27.0, 1.0], vec![true, false, true]);

    let out = v.cbrt();

    assert_eq!(out.data, vec![2.0, -3.0, 1.0]);
    // validity unchanged
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_f64_cbrt_inplace() {
    let mut v = vd_f64(vec![8.0, -27.0], vec![true, true]);

    v.cbrt_inplace();

    assert_eq!(v.data, vec![2.0, -3.0]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_i64_rsqrt_owned_basic() {
    let v = vd_i64(vec![1, 4, 16], vec![true; 3]);

    let out = v.rsqrt();

    assert_eq!(out.data, vec![1.0, 0.5, 0.25]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_i64_rsqrt_owned_invalid_negative() {
    let v = vd_i64(vec![4, -9, 16], vec![true; 3]);

    let out = v.rsqrt();

    assert_eq!(out.data[0], 0.5);
    assert_eq!(out.data[2], 0.25);

    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_f64_rsqrt_owned_basic() {
    let v = vd_f64(vec![1.0, 4.0, 16.0], vec![true;3]);

    let out = v.rsqrt();

    assert_eq!(out.data, vec![1.0, 0.5, 0.25]);
}

#[test]
fn test_f64_rsqrt_inplace() {
    let mut v = vd_f64(vec![1.0, 4.0, -9.0], vec![true;3]);

    v.rsqrt_inplace();

    assert_eq!(v.data[0], 1.0);
    assert_eq!(v.data[1], 0.5);

    assert_eq!(v.validity.to_vec(), bitvec![1,1,0]);
}

#[test]
fn test_f64_rsqrt_range_full() {
    let v = vd_f64(vec![1.0, 4.0, 9.0, 16.0], vec![true;4]);

    let out = v.rsqrt_range(1, 3, true);

    assert_eq!(out.data, vec![1.0, 0.5, 1.0/9.0_f64.sqrt(), 16.0]);
}

#[test]
fn test_i64_rcbrt_owned_basic() {
    let v = vd_i64(vec![1, 8, 27], vec![true;3]);

    let out = v.rcbrt();

    assert_eq!(out.data, vec![1.0, 0.5, 1.0 / 3.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1,1,1]);
}

#[test]
fn test_i64_rcbrt_owned_zero() {
    let v = vd_i64(vec![0, 8], vec![true;2]);

    let out = v.rcbrt();

    assert_eq!(out.data[0], f64::INFINITY);
    assert_eq!(out.data[1], 0.5);
}

#[test]
fn test_f64_rcbrt_owned_basic() {
    let v = vd_f64(vec![1.0, 8.0, 27.0], vec![true;3]);

    let out = v.rcbrt();

    assert_eq!(out.data, vec![1.0, 0.5, 1.0/3.0]);
}

#[test]
fn test_f64_rcbrt_inplace() {
    let mut v = vd_f64(vec![1.0, 8.0, 0.0], vec![true;3]);

    v.rcbrt_inplace();

    assert_eq!(v.data[0], 1.0);
    assert_eq!(v.data[1], 0.5);
    assert_eq!(v.data[2], f64::INFINITY);
}

#[test]
fn test_f64_rcbrt_range_full() {
    let v = vd_f64(vec![1.0, 8.0, 27.0], vec![true;3]);

    let out = v.rcbrt_range(1, 3, true);

    assert_eq!(out.data, vec![1.0, 0.5, 1.0/3.0]);
}

#[test]
fn test_i64_nth_root_owned_basic() {
    let v = vd_i64(vec![1, 8, 27], vec![true, true, true]);

    let out = v.nth_root(3.0);

    let expected = vec![1.0_f64, 2.0_f64, 3.0_f64];

    assert_eq!(out.data, expected);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_i64_nth_root_negatives() {
    let v = vd_i64(vec![-8, -27, 16], vec![true, true, true]);

    // 3rd root → valid; 2nd root → invalid
    let out = v.nth_root(2.0); // even root → negative values → NaN

    assert_eq!(out.data[2], 4.0);

    assert_eq!(out.validity.to_vec(), bitvec![0, 0, 1]);
}

#[test]
fn test_i64_nth_root_fractional_owned() {
    let v = vd_i64(vec![4, 9, 16], vec![true, true, true]);

    let out = v.nth_root(2.0); // same as sqrt

    assert_eq!(out.data, vec![2.0, 3.0, 4.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_i64_nth_root_range_slice_only() {
    let v = vd_i64(vec![1, 8, 27, 64], vec![true, true, true, true]);

    let out = v.nth_root_range(3.0, 1, 3, false);

    assert_eq!(out.data, vec![2.0, 3.0]); // cube roots of 8 and 27
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_i64_nth_root_range_full() {
    let v = vd_i64(vec![1, 8, 27, 64], vec![true, true, true, true]);

    let out = v.nth_root_range(3.0, 1, 3, true);

    assert_eq!(out.data, vec![
        1.0,  // unchanged outside slice
        2.0,  // 8^(1/3)
        3.0,  // 27^(1/3)
        64.0  // converted (no root applied)
    ]);

    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_f64_nth_root_owned_basic() {
    let v = vd_f64(vec![1.0, 8.0, 27.0], vec![true, true, true]);

    let out = v.nth_root(3.0);

    assert_eq!(out.data, vec![1.0, 2.0, 3.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_f64_nth_root_nan_propagation() {
    let v = vd_f64(vec![-8.0, 4.0, -27.0], vec![true, true, true]);

    let out = v.nth_root(2.0); // even root produces NaN on negatives

    assert_eq!(out.data[1], 2.0);

    assert_eq!(out.validity.to_vec(), bitvec![0, 1, 0]);
}

#[test]
fn test_f64_nth_root_inplace_basic() {
    let mut v = vd_f64(vec![1.0, 8.0, 27.0], vec![true, true, true]);

    v.nth_root_inplace(3.0);

    assert_eq!(v.data, vec![1.0, 2.0, 3.0]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_f64_nth_root_inplace_invalid() {
    let mut v = vd_f64(vec![-8.0, 4.0, -27.0], vec![true, true, true]);

    v.nth_root_inplace(2.0);

    assert_eq!(v.data[1], 2.0);

    assert_eq!(v.validity.to_vec(), bitvec![0, 1, 0]);
}

#[test]
fn test_f64_nth_root_range_slice_only() {
    let v = vd_f64(vec![1.0, 8.0, 27.0, 64.0], vec![true; 4]);

    let out = v.nth_root_range(3.0, 1, 3, false);

    assert_eq!(out.data, vec![2.0, 3.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_f64_nth_root_range_full() {
    let v = vd_f64(vec![1.0, 8.0, 27.0, 64.0], vec![true; 4]);

    let out = v.nth_root_range(3.0, 1, 3, true);

    assert_eq!(out.data, vec![
        1.0,  // unchanged
        2.0,  // nth_root applied
        3.0,  // nth_root applied
        64.0  // converted as identity
    ]);

    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_i64_mul_scalar_owned() {
    let v = vd_i64(vec![1, -2, 3], vec![true, false, true]);
    let out = v.mul_scalar(10.0);

    assert_eq!(out.data, vec![10.0, -20.0, 30.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_i64_mul_scalar_range_slice() {
    let v = vd_i64(vec![1, 2, 3, 4], vec![true; 4]);
    let out = v.mul_scalar_range(10.0, 1, 3, false);

    assert_eq!(out.data, vec![20.0, 30.0]);
}

#[test]
fn test_i64_mul_scalar_range_full() {
    let v = vd_i64(vec![1, 2, 3, 4], vec![true; 4]);
    let out = v.mul_scalar_range(10.0, 1, 3, true);

    assert_eq!(out.data, vec![1.0, 20.0, 30.0, 4.0]);
}

#[test]
fn test_i64_add_scalar_owned() {
    let v = vd_i64(vec![1, -2, 3], vec![true, false, true]);
    let out = v.add_scalar(5.0);

    assert_eq!(out.data, vec![6.0, 3.0, 8.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_i64_add_scalar_range_full() {
    let v = vd_i64(vec![1, 2, 3], vec![true; 3]);
    let out = v.add_scalar_range(2.0, 1, 3, true);

    assert_eq!(out.data, vec![1.0, 4.0, 5.0]);
}

#[test]
fn test_i64_sub_scalar_owned() {
    let v = vd_i64(vec![10, 5, -3], vec![true, true, false]);
    let out = v.sub_scalar(2.0);

    assert_eq!(out.data, vec![8.0, 3.0, -5.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 0]);
}

#[test]
fn test_i64_sub_scalar_range_full() {
    let v = vd_i64(vec![10, 20, 30], vec![true; 3]);
    let out = v.sub_scalar_range(5.0, 1, 3, true);

    assert_eq!(out.data, vec![10.0, 15.0, 25.0]);
}

#[test]
fn test_i64_div_scalar_owned() {
    let v = vd_i64(vec![10, 0, -5], vec![true, true, true]);
    let out = v.div_scalar(2.0);

    assert_eq!(out.data, vec![5.0, 0.0, -2.5]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_i64_div_scalar_zero_div_zero() {
    let v = vd_i64(vec![0], vec![true]);
    let out = v.div_scalar(0.0);

    assert_eq!(out.validity.to_vec(), bitvec![0]);
}

#[test]
fn test_i64_inv_div_scalar_owned() {
    let v = vd_i64(vec![2, -2, 0], vec![true; 3]);
    let out = v.inv_div_scalar(10.0);

    assert_eq!(out.data, vec![5.0, -5.0, f64::INFINITY]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_i64_inv_div_scalar_zero_zero() {
    let v = vd_i64(vec![0], vec![true]);
    let out = v.inv_div_scalar(0.0);

    assert_eq!(out.validity.to_vec(), bitvec![0]);
}

#[test]
fn test_f64_mul_scalar_owned() {
    let v = vd_f64(vec![1.0, -2.0, 3.0], vec![true, false, true]);
    let out = v.mul_scalar(2.0);

    assert_eq!(out.data, vec![2.0, -4.0, 6.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_f64_mul_scalar_inplace() {
    let mut v = vd_f64(vec![1.0, 2.0], vec![true; 2]);
    v.mul_scalar_inplace(3.0);

    assert_eq!(v.data, vec![3.0, 6.0]);
}

#[test]
fn test_f64_add_scalar_owned() {
    let v = vd_f64(vec![1.0, 2.0, -1.0], vec![true; 3]);
    let out = v.add_scalar(5.5);

    assert_eq!(out.data, vec![6.5, 7.5, 4.5]);
}

#[test]
fn test_f64_sub_scalar_owned() {
    let v = vd_f64(vec![10.0, 5.0], vec![true; 2]);
    let out = v.sub_scalar(1.5);

    assert_eq!(out.data, vec![8.5, 3.5]);
}

#[test]
fn test_f64_div_scalar_basic() {
    let v = vd_f64(vec![10.0, -4.0], vec![true; 2]);
    let out = v.div_scalar(2.0);

    assert_eq!(out.data, vec![5.0, -2.0]);
}

#[test]
fn test_f64_div_scalar_zero_zero() {
    let v = vd_f64(vec![0.0], vec![true]);
    let out = v.div_scalar(0.0);

    assert_eq!(out.validity.to_vec(), bitvec![0]);
}

#[test]
fn test_f64_inv_div_scalar_owned() {
    let v = vd_f64(vec![2.0, -2.0, 0.0], vec![true; 3]);
    let out = v.inv_div_scalar(4.0);

    assert_eq!(out.data, vec![2.0, -2.0, f64::INFINITY]);
}

#[test]
fn test_f64_inv_div_scalar_zero_zero() {
    let v = vd_f64(vec![0.0], vec![true]);
    let out = v.inv_div_scalar(0.0);

    assert_eq!(out.validity.to_vec(), bitvec![0]);
}

#[test]
fn test_i64_add_scalar_operator() {
    let v = vd_i64(vec![1, 2, -3], vec![true, false, true]);
    let out = v + 5.0;

    assert_eq!(out.data, vec![6.0, 7.0, 2.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_i64_mul_scalar_operator_ref() {
    let v = vd_i64(vec![2, -1], vec![true, true]);
    let out = &v * 3.0;
    assert_eq!(out.data, vec![6.0, -3.0]);
}

#[test]
fn test_f64_div_scalar_operator() {
    let v = vd_f64(vec![10.0, -4.0], vec![true; 2]);
    let out = v / 2.0;
    assert_eq!(out.data, vec![5.0, -2.0]);
}

#[test]
fn test_f64_scalar_div_vector() {
    let v = vd_f64(vec![2.0, -2.0, 0.0], vec![true; 3]);
    let out = 10.0 / v;

    assert_eq!(out.data[0], 5.0);
    assert_eq!(out.data[1], -5.0);
    assert!(out.data[2].is_infinite());
}

#[test]
fn test_f64_powf_owned() {
    let v = vd_f64(vec![2.0, 4.0, 9.0], vec![true; 3]);
    let out = v.powf(0.5);

    assert_eq!(out.data, vec![2.0_f64.sqrt(), 4.0_f64.sqrt(), 9.0_f64.sqrt()]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_f64_powf_inplace() {
    let mut v = vd_f64(vec![2.0, 4.0], vec![true; 2]);
    v.powf_inplace(2.0);

    assert_eq!(v.data, vec![4.0, 16.0]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_f64_powf_range_full() {
    let v = vd_f64(vec![1.0, 2.0, 3.0], vec![true; 3]);
    let out = v.powf_range(2.0, 1, 3, true);

    assert_eq!(out.data, vec![1.0, 4.0, 9.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_f64_powi_owned() {
    let v = vd_f64(vec![2.0, 3.0], vec![true; 2]);
    let out = v.powi(3);

    assert_eq!(out.data, vec![8.0, 27.0]);
}

#[test]
fn test_f64_powi_inplace() {
    let mut v = vd_f64(vec![2.0, 3.0], vec![true; 2]);
    v.powi_inplace(2);

    assert_eq!(v.data, vec![4.0, 9.0]);
}

#[test]
fn test_f64_powi_range_slice_only() {
    let v = vd_f64(vec![2.0, 3.0, 4.0], vec![true; 3]);
    let out = v.powi_range(2, 1, 3, false);

    assert_eq!(out.data, vec![9.0, 16.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_i64_powf_owned() {
    let v = vd_i64(vec![4, 9], vec![true; 2]);
    let out = v.powf(0.5);

    assert_eq!(out.data, vec![2.0, 3.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_i64_powf_range_full() {
    let v = vd_i64(vec![1, 4, 9], vec![true; 3]);
    let out = v.powf_range(0.5, 1, 3, true);

    assert_eq!(out.data, vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_i64_powi_owned() {
    let v = vd_i64(vec![2, 3], vec![true; 2]);
    let out = v.powi(3);

    assert_eq!(out.data, vec![8.0, 27.0]);
}

#[test]
fn test_i64_powi_range_slice_only() {
    let v = vd_i64(vec![2, 3, 4], vec![true; 3]);
    let out = v.powi_range(2, 1, 3, false);

    assert_eq!(out.data, vec![9.0, 16.0]);
}

#[test]
fn test_i64_powi_pos_owned() {
    let v = vd_i64(vec![2, 3], vec![true; 2]);
    let out = v.powi_pos(3);

    assert_eq!(out.data, vec![8, 27]);
}

#[test]
fn test_i64_powi_pos_inplace() {
    let mut v = vd_i64(vec![2, 3], vec![true; 2]);
    v.powi_pos_inplace(2);

    assert_eq!(v.data, vec![4, 9]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_i64_powi_pos_range_full() {
    let v = vd_i64(vec![1, 2, 3], vec![true; 3]);
    let out = v.powi_pos_range(2, 1, 3, true);

    assert_eq!(out.data, vec![1, 4, 9]);
}

#[test]
fn test_i64_powi_pos_range_slice_only() {
    let v = vd_i64(vec![1, 2, 3], vec![true; 3]);
    let out = v.powi_pos_range(2, 1, 3, false);

    assert_eq!(out.data, vec![4, 9]);
}

#[test]
fn test_f64_logs_owned() {
    let input = vec![1.0, 2.0, 10.0, -1.0, 0.0];
    let validity = vec![true; 5];

    let v = vd_f64(input.clone(), validity.clone());

    assert_vd_f64_matches(&v.ln(),      &input, &validity, |x| x.ln());
    assert_vd_f64_matches(&v.ln_1p(),   &input, &validity, |x| x.ln_1p());
    assert_vd_f64_matches(&v.log(),     &input, &validity, |x| x.ln());
    assert_vd_f64_matches(&v.log2(),    &input, &validity, |x| x.log2());
    assert_vd_f64_matches(&v.log10(),   &input, &validity, |x| x.log10());
    assert_vd_f64_matches(&v.logb(3.0),    &input, &validity, |x| x.log(3.0));
}

#[test]
fn test_f64_logs_inplace() {
    let input = vec![1.0, 2.0, 10.0, -1.0, 0.0];
    let validity = vec![true; 5];

    let mut v;

    v = vd_f64(input.clone(), validity.clone());
    v.ln_inplace();
    assert_vd_f64_matches(&v, &input, &validity, |x| x.ln());

    v = vd_f64(input.clone(), validity.clone());
    v.ln_1p_inplace();
    assert_vd_f64_matches(&v, &input, &validity, |x| x.ln_1p());

    v = vd_f64(input.clone(), validity.clone());
    v.log2_inplace();
    assert_vd_f64_matches(&v, &input, &validity, |x| x.log2());

    v = vd_f64(input.clone(), validity.clone());
    v.log10_inplace();
    assert_vd_f64_matches(&v, &input, &validity, |x| x.log10());

    v = vd_f64(input.clone(), validity.clone());
    v.logb_inplace(3.0);
    assert_vd_f64_matches(&v, &input, &validity, |x| x.log(3.0));
}

#[test]
fn test_f64_logs_range_slice() {
    let input = vec![1.0, 2.0, 10.0, -1.0, 0.0];
    let validity = vec![true; 5];

    let v = vd_f64(input.clone(), validity.clone());

    let out = v.ln_range(1, 4, false);
    assert_vd_f64_matches(&out, &input[1..4], &validity[1..4], |x| x.ln());

    let out = v.log2_range(1, 4, false);
    assert_vd_f64_matches(&out, &input[1..4], &validity[1..4], |x| x.log2());
}

#[test]
fn test_f64_logs_range_full() {
    let input = vec![1.0, 2.0, 10.0, -1.0, 0.0];
    let validity = vec![true; 5];

    let v = vd_f64(input.clone(), validity.clone());

    let out = v.log10_range(1, 4, true);
    assert_vd_f64_matches_range(&out, &input, &validity, 1,4,|x| x.log10());
}

#[test]
fn test_i64_logs_owned() {
    let input_i64 = vec![1, 2, 10, -1, 0];
    let input_f64: Vec<f64> = input_i64.iter().map(|&x| x as f64).collect();
    let validity = vec![true; 5];

    let v = vd_i64(input_i64, validity.clone());

    assert_vd_f64_matches(&v.ln(),    &input_f64, &validity, |x| x.ln());
    assert_vd_f64_matches(&v.log2(),  &input_f64, &validity, |x| x.log2());
    assert_vd_f64_matches(&v.log10(), &input_f64, &validity, |x| x.log10());
    assert_vd_f64_matches(&v.logb(3.0),  &input_f64, &validity, |x| x.log(3.0));
}

#[test]
fn test_i64_logs_range_slice() {
    let input_i64 = vec![1, 2, 10, -1, 0];
    let input_f64: Vec<f64> = input_i64.iter().map(|&x| x as f64).collect();
    let validity = vec![true; 5];

    let v = vd_i64(input_i64, validity.clone());

    let out = v.ln_range(1, 4, false);
    assert_vd_f64_matches(&out, &input_f64[1..4], &validity[1..4], |x| x.ln());
}

#[test]
fn test_i64_logs_range_full() {
    let input_i64 = vec![1, 2, 10, -1, 0];
    let input_f64: Vec<f64> = input_i64.iter().map(|&x| x as f64).collect();
    let validity = vec![true; 5];

    let v = vd_i64(input_i64, validity.clone());

    let out = v.log2_range(1, 4, true);
    assert_vd_f64_matches_range(&out, &input_f64, &validity, 1, 4, |x| x.log2());
}

#[test]
fn test_i64_clip_owned_basic() {
    let v = vd_i64(vec![1, 5, 10, -3], vec![true; 4]);

    let out = v.clip(0, 6);

    assert_eq!(out.data, vec![1, 5, 6, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_i64_clip_owned_with_nulls() {
    let v = vd_i64(vec![1, 5, 10, -3], vec![true, false, true, true]);

    let out = v.clip(0, 6);

    assert_eq!(out.data, vec![1, 5, 6, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1, 1]);
}

#[test]
fn test_i64_clip_owned_lo_gt_hi() {
    let v = vd_i64(vec![1, 2, 3], vec![true; 3]);

    let out = v.clip(5, 1);

    assert_eq!(out.data, vec![0, 0, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 0, 0]);
}

#[test]
fn test_i64_clip_inplace_basic() {
    let mut v = vd_i64(vec![1, 5, 10, -3], vec![true; 4]);

    v.clip_inplace(0, 6);

    assert_eq!(v.data, vec![1, 5, 6, 0]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_i64_clip_inplace_with_nulls() {
    let mut v = vd_i64(vec![1, 5, 10, -3], vec![true, false, true, true]);

    v.clip_inplace(0, 6);

    assert_eq!(v.data, vec![1, 5, 6, 0]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 0, 1, 1]);
}

#[test]
fn test_i64_clip_inplace_lo_gt_hi() {
    let mut v = vd_i64(vec![1, 2, 3], vec![true; 3]);

    v.clip_inplace(5, 1);

    assert_eq!(v.data, vec![0, 0, 0]);
    assert_eq!(v.validity.to_vec(), bitvec![0, 0, 0]);
}

#[test]
fn test_i64_clip_range_slice_only() {
    let v = vd_i64(vec![1, 5, 10, -3], vec![true; 4]);

    let out = v.clip_range(0, 6, 1, 3, false);

    assert_eq!(out.data, vec![5, 6]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_i64_clip_range_full() {
    let v = vd_i64(vec![1, 5, 10, -3], vec![true; 4]);

    let out = v.clip_range(0, 6, 1, 3, true);

    assert_eq!(out.data, vec![1, 5, 6, -3]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_i64_clip_range_full_lo_gt_hi() {
    let v = vd_i64(vec![1, 2, 3, 4], vec![true; 4]);

    let out = v.clip_range(5, 1, 1, 3, true);

    assert_eq!(out.data, vec![1, 0, 0, 4]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 0, 1]);
}

#[test]
fn test_f64_clip_owned_basic() {
    let v = vd_f64(vec![-1.0, 0.5, 3.0, 10.0], vec![true; 4]);

    let out = v.clip(0.0, 5.0);

    assert_eq!(out.data, vec![0.0, 0.5, 3.0, 5.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_f64_clip_owned_with_nulls() {
    let v = vd_f64(vec![-1.0, 0.5, 3.0, 10.0], vec![true, false, true, true]);

    let out = v.clip(0.0, 5.0);

    assert_eq!(out.data, vec![0.0, 0.5, 3.0, 5.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1, 1]);
}

#[test]
fn test_f64_clip_owned_lo_gt_hi() {
    let v = vd_f64(vec![1.0, 2.0, 3.0], vec![true; 3]);

    let out = v.clip(5.0, 1.0);

    assert_eq!(out.data, vec![0.0, 0.0, 0.0]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 0, 0]);
}

#[test]
fn test_f64_clip_inplace_basic() {
    let mut v = vd_f64(vec![-1.0, 0.5, 3.0, 10.0], vec![true; 4]);

    v.clip_inplace(0.0, 5.0);

    assert_eq!(v.data, vec![0.0, 0.5, 3.0, 5.0]);
    assert_eq!(v.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_f64_clip_inplace_lo_gt_hi() {
    let mut v = vd_f64(vec![1.0, 2.0, 3.0], vec![true; 3]);

    v.clip_inplace(5.0, 1.0);

    assert_eq!(v.data, vec![0.0, 0.0, 0.0]);
    assert_eq!(v.validity.to_vec(), bitvec![0, 0, 0]);
}

#[test]
fn test_f64_clip_range_slice_only() {
    let v = vd_f64(vec![-1.0, 0.5, 3.0, 10.0], vec![true; 4]);

    let out = v.clip_range(0.0, 5.0, 1, 3, false);

    assert_eq!(out.data, vec![0.5, 3.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_f64_clip_range_full() {
    let v = vd_f64(vec![-1.0, 0.5, 3.0, 10.0], vec![true; 4]);

    let out = v.clip_range(0.0, 5.0, 1, 3, true);

    assert_eq!(out.data, vec![-1.0, 0.5, 3.0, 10.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_f64_clip_range_full_lo_gt_hi() {
    let v = vd_f64(vec![1.0, 2.0, 3.0, 4.0], vec![true; 4]);

    let out = v.clip_range(5.0, 1.0, 1, 3, true);

    assert_eq!(out.data, vec![1.0, 0.0, 0.0, 4.0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 0, 1]);
}

#[test]
fn test_i64_sin_owned() {
    let input = vec![0, 1, -2, 3];
    let validity = vec![true; 4];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.sin();

    let expected: Vec<f64> = input.iter().map(|&x| (x as f64).sin()).collect();
    assert_vec_approx(&out.data, &expected);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_i64_sin_range_full() {
    let input = vec![0, 1, -2, 3];
    let validity = vec![true; 4];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.sin_range(1, 3, true);

    assert_vd_f64_matches_range(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        1,
        3,
        |x| x.sin(),
    );
}

#[test]
fn test_i64_cos_owned() {
    let input = vec![0, 1, -2, 3];
    let validity = vec![true; 4];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.cos();

    let expected: Vec<f64> = input.iter().map(|&x| (x as f64).cos()).collect();
    assert_vec_approx(&out.data, &expected);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_i64_cos_range_full() {
    let input = vec![0, 1, -2, 3];
    let validity = vec![true; 4];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.cos_range(1, 3, true);

    assert_vd_f64_matches_range(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        1,
        3,
        |x| x.cos(),
    );
}

#[test]
fn test_i64_tan_owned() {
    let input = vec![0, 1, 2];
    let validity = vec![true; 3];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.tan();

    assert_vd_f64_matches(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        |x| x.tan(),
    );
}

#[test]
fn test_i64_tan_range_full() {
    let input = vec![0, 1, 2, 3];
    let validity = vec![true; 4];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.tan_range(1, 3, true);

    assert_vd_f64_matches_range(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        1,
        3,
        |x| x.tan(),
    );
}

#[test]
fn test_f64_sin_owned() {
    let input = vec![0.0, 1.2, -2.5, 3.1];
    let validity = vec![true; 4];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.sin();

    assert_vd_f64_matches(&out, &input, &validity, |x| x.sin());
}

#[test]
fn test_f64_sin_inplace() {
    let input = vec![0.0, 1.2, -2.5, 3.1];
    let validity = vec![true; 4];

    let mut v = vd_f64(input.clone(), validity.clone());
    v.sin_inplace();

    assert_vd_f64_matches(&v, &input, &validity, |x| x.sin());
}

#[test]
fn test_f64_sin_range_full() {
    let input = vec![0.0, 1.2, -2.5, 3.1];
    let validity = vec![true; 4];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.sin_range(1, 3, true);

    assert_vd_f64_matches_range(&out, &input, &validity, 1, 3, |x| x.sin());
}

#[test]
fn test_f64_cos_owned() {
    let input = vec![0.0, 1.2, -2.5, 3.1];
    let validity = vec![true; 4];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.cos();

    assert_vd_f64_matches(&out, &input, &validity, |x| x.cos());
}

#[test]
fn test_f64_cos_inplace() {
    let input = vec![0.0, 1.2, -2.5, 3.1];
    let validity = vec![true; 4];

    let mut v = vd_f64(input.clone(), validity.clone());
    v.cos_inplace();

    assert_vd_f64_matches(&v, &input, &validity, |x| x.cos());
}

#[test]
fn test_f64_cos_range_full() {
    let input = vec![0.0, 1.2, -2.5, 3.1];
    let validity = vec![true; 4];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.cos_range(1, 3, true);

    assert_vd_f64_matches_range(&out, &input, &validity, 1, 3, |x| x.cos());
}

#[test]
fn test_f64_tan_owned() {
    let input = vec![0.0, 1.2, -2.5, 3.1];
    let validity = vec![true; 4];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.tan();

    assert_vd_f64_matches(&out, &input, &validity, |x| x.tan());
}

#[test]
fn test_f64_tan_inplace() {
    let input = vec![0.0, 1.2, -2.5, 3.1];
    let validity = vec![true; 4];

    let mut v = vd_f64(input.clone(), validity.clone());
    v.tan_inplace();

    assert_vd_f64_matches(&v, &input, &validity, |x| x.tan());
}

#[test]
fn test_f64_tan_range_full() {
    let input = vec![0.0, 1.2, -2.5, 3.1];
    let validity = vec![true; 4];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.tan_range(1, 3, true);

    assert_vd_f64_matches_range(&out, &input, &validity, 1, 3, |x| x.tan());
}

#[test]
fn test_i64_asin_owned() {
    let input = vec![-1, 0, 1, 2];
    let validity = vec![true; 4];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.asin();

    assert_vd_f64_matches(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        |x| x.asin(),
    );
}

#[test]
fn test_i64_acos_range_full() {
    let input = vec![-1, 0, 1, 2];
    let validity = vec![true; 4];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.acos_range(1, 4, true);

    assert_vd_f64_matches_range(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        1,
        4,
        |x| x.acos(),
    );
}

#[test]
fn test_i64_atan_owned() {
    let input = vec![-10, -1, 0, 1, 10];
    let validity = vec![true; 5];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.atan();

    assert_vd_f64_matches(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        |x| x.atan(),
    );
}

#[test]
fn test_f64_asin_owned() {
    let input = vec![-1.0, 0.0, 1.0, 1.5];
    let validity = vec![true; 4];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.asin();

    assert_vd_f64_matches(&out, &input, &validity, |x| x.asin());
}

#[test]
fn test_f64_acos_inplace() {
    let input = vec![-1.0, 0.0, 1.0, 1.5];
    let validity = vec![true; 4];

    let mut v = vd_f64(input.clone(), validity.clone());
    v.acos_inplace();

    assert_vd_f64_matches(&v, &input, &validity, |x| x.acos());
}

#[test]
fn test_f64_atan_range_full() {
    let input = vec![-10.0, -1.0, 0.0, 1.0, 10.0];
    let validity = vec![true; 5];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.atan_range(1, 4, true);

    assert_vd_f64_matches_range(&out, &input, &validity, 1, 4, |x| x.atan());
}

#[test]
fn test_i64_sinh_owned() {
    let input = vec![-5, -1, 0, 1, 5];
    let validity = vec![true; 5];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.sinh();

    assert_vd_f64_matches(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        |x| x.sinh(),
    );
}

#[test]
fn test_i64_cosh_range_full() {
    let input = vec![-5, -1, 0, 1, 5];
    let validity = vec![true; 5];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.cosh_range(1, 4, true);

    assert_vd_f64_matches_range(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        1,
        4,
        |x| x.cosh(),
    );
}

#[test]
fn test_i64_tanh_owned() {
    let input = vec![-10, -1, 0, 1, 10];
    let validity = vec![true; 5];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.tanh();

    assert_vd_f64_matches(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        |x| x.tanh(),
    );
}

#[test]
fn test_f64_sinh_owned() {
    let input = vec![-3.0, -1.0, 0.0, 1.0, 3.0];
    let validity = vec![true; 5];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.sinh();

    assert_vd_f64_matches(&out, &input, &validity, |x| x.sinh());
}

#[test]
fn test_f64_cosh_inplace() {
    let input = vec![-3.0, -1.0, 0.0, 1.0, 3.0];
    let validity = vec![true; 5];

    let mut v = vd_f64(input.clone(), validity.clone());
    v.cosh_inplace();

    assert_vd_f64_matches(&v, &input, &validity, |x| x.cosh());
}

#[test]
fn test_f64_tanh_range_full() {
    let input = vec![-10.0, -1.0, 0.0, 1.0, 10.0];
    let validity = vec![true; 5];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.tanh_range(1, 4, true);

    assert_vd_f64_matches_range(&out, &input, &validity, 1, 4, |x| x.tanh());
}

#[test]
fn test_i64_asinh_owned() {
    let input = vec![-10, -1, 0, 1, 10];
    let validity = vec![true; 5];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.asinh();

    assert_vd_f64_matches(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        |x| x.asinh(),
    );
}

#[test]
fn test_i64_acosh_range_full() {
    let input = vec![0, 1, 2, 10];
    let validity = vec![true; 4];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.acosh_range(1, 4, true);

    assert_vd_f64_matches_range(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        1,
        4,
        |x| x.acosh(),
    );
}

#[test]
fn test_i64_atanh_owned() {
    let input = vec![-2, -1, 0, 1, 2];
    let validity = vec![true; 5];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.atanh();

    assert_vd_f64_matches(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        |x| x.atanh(),
    );
}

#[test]
fn test_f64_asinh_inplace() {
    let input = vec![-10.0, -1.0, 0.0, 1.0, 10.0];
    let validity = vec![true; 5];

    let mut v = vd_f64(input.clone(), validity.clone());
    v.asinh_inplace();

    assert_vd_f64_matches(&v, &input, &validity, |x| x.asinh());
}

#[test]
fn test_f64_acosh_owned() {
    let input = vec![0.5, 1.0, 2.0, 10.0];
    let validity = vec![true; 4];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.acosh();

    assert_vd_f64_matches(&out, &input, &validity, |x| x.acosh());
}

#[test]
fn test_f64_atanh_range_full() {
    let input = vec![-2.0, -0.5, 0.0, 0.5, 2.0];
    let validity = vec![true; 5];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.atanh_range(1, 4, true);

    assert_vd_f64_matches_range(&out, &input, &validity, 1, 4, |x| x.atanh());
}

#[test]
fn test_f64_cut_unbounded_right() {
    let v = vd_f64(
        vec![-2.0, -1.0, 0.0, 1.0, 2.0],
        vec![true; 5],
    );

    let out = v.cut(&[-1.0, 1.0], true, false).unwrap();

    assert_eq!(out.data, vec![0, 0, 1, 1, 2]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1, 1]);
}

#[test]
fn test_f64_cut_unbounded_left() {
    let v = vd_f64(
        vec![-2.0, -1.0, 0.0, 1.0, 2.0],
        vec![true; 5],
    );

    let out = v.cut(&[-1.0, 1.0], false, false).unwrap();

    assert_eq!(out.data, vec![0, 1, 1, 2, 2]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1, 1]);
}

#[test]
fn test_f64_cut_bounded_right() {
    let v = vd_f64(
        vec![-2.0, -1.0, 0.0, 1.0, 2.0],
        vec![true; 5],
    );

    let out = v.cut(&[-1.0, 1.0], true, true).unwrap();

    assert_eq!(out.data, vec![0, 0, 0, 0, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 0, 1, 1, 0]);
}

#[test]
fn test_f64_cut_bounded_left() {
    let v = vd_f64(
        vec![-2.0, -1.0, 0.0, 1.0, 2.0],
        vec![true; 5],
    );

    let out = v.cut(&[-1.0, 1.0], false, true).unwrap();

    assert_eq!(out.data, vec![0, 0, 0, 0, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1, 1, 0, 0]);
}

#[test]
fn test_f64_cut_preserves_existing_na() {
    let v = vd_f64(
        vec![-2.0, 0.0, 2.0],
        vec![true, false, true],
    );

    let out = v.cut(&[-1.0, 1.0], true, false).unwrap();

    assert_eq!(out.data, vec![0, 1, 2]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_i64_cut_unbounded_right() {
    let v = vd_i64(
        vec![-2, -1, 0, 1, 2],
        vec![true; 5],
    );

    let out = v.cut(&[-1.0, 1.0], true, false).unwrap();

    assert_eq!(out.data, vec![0, 0, 1, 1, 2]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1, 1]);
}

#[test]
fn test_i64_cut_unbounded_left() {
    let v = vd_i64(
        vec![-2, -1, 0, 1, 2],
        vec![true; 5],
    );

    let out = v.cut(&[-1.0, 1.0], false, false).unwrap();

    assert_eq!(out.data, vec![0, 1, 1, 2, 2]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1, 1]);
}

#[test]
fn test_i64_cut_bounded_right() {
    let v = vd_i64(
        vec![-2, -1, 0, 1, 2],
        vec![true; 5],
    );

    let out = v.cut(&[-1.0, 1.0], true, true).unwrap();

    assert_eq!(out.data, vec![0, 0, 0, 0, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 0, 1, 1, 0]);
}

#[test]
fn test_i64_cut_bounded_left() {
    let v = vd_i64(
        vec![-2, -1, 0, 1, 2],
        vec![true; 5],
    );

    let out = v.cut(&[-1.0, 1.0], false, true).unwrap();

    assert_eq!(out.data, vec![0, 0, 0, 0, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1, 1, 0, 0]);
}

#[test]
fn test_i64_cut_preserves_existing_na() {
    let v = vd_i64(
        vec![-2, 0, 2],
        vec![true, false, true],
    );

    let out = v.cut(&[-1.0, 1.0], true, false).unwrap();

    assert_eq!(out.data, vec![0, 1, 2]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_f64_cut_labels_unbounded_right() {
    let v = vd_f64(vec![-2.0, -1.0, 0.0, 1.0, 2.0], vec![true; 5]);
    let bins = vec![-1.0, 1.0];
    let labels = vec!["A", "B", "C"].into_iter().map(String::from).collect::<Vec<_>>();

    let out = v.cut_labels(&bins, &labels, true, false).unwrap();

    assert_eq!(
        out.data,
        vec!["A", "A", "B", "B", "C"]
    );
    assert_eq!(out.validity.to_vec(), bitvec![1,1,1,1,1]);
}

#[test]
fn test_f64_cut_labels_unbounded_left() {
    let v = vd_f64(vec![-1.0, 0.0, 1.0], vec![true; 3]);
    let bins = vec![-1.0, 1.0];
    let labels = vec!["A", "B", "C"].into_iter().map(String::from).collect::<Vec<_>>();

    let out = v.cut_labels(&bins, &labels, false, false).unwrap();

    assert_eq!(
        out.data,
        vec!["B", "B", "C"]
    );
    assert_eq!(out.validity.to_vec(), bitvec![1,1,1]);
}

#[test]
fn test_f64_cut_labels_bounded_right() {
    let v = vd_f64(vec![-2.0, -1.0, 0.0, 1.0, 2.0], vec![true; 5]);
    let bins = vec![-1.0, 1.0];
    let labels = vec!["MID"].into_iter().map(String::from).collect::<Vec<_>>();

    let out = v.cut_labels(&bins, &labels, true, true).unwrap();

    assert_eq!(
        out.data,
        vec!["MID", "MID", "MID", "MID", "MID"]
    );
    assert_eq!(out.validity.to_vec(), bitvec![0,0,1,1,0]);
}

#[test]
fn test_f64_cut_labels_bounded_left() {
    let v = vd_f64(vec![-2.0, -1.0, 0.0, 1.0, 2.0], vec![true; 5]);
    let bins = vec![-1.0, 1.0];
    let labels = vec!["MID"].into_iter().map(String::from).collect::<Vec<_>>();

    let out = v.cut_labels(&bins, &labels, false, true).unwrap();

    assert_eq!(
        out.data,
        vec!["MID", "MID", "MID", "MID", "MID"]
    );
    assert_eq!(out.validity.to_vec(), bitvec![0,1,1,0,0]);
}

#[test]
fn test_i64_cut_labels_unbounded_right() {
    let v = vd_i64(vec![-2, -1, 0, 1, 2], vec![true; 5]);
    let bins = vec![-1.0, 1.0];
    let labels = vec!["A", "B", "C"].into_iter().map(String::from).collect::<Vec<_>>();

    let out = v.cut_labels(&bins, &labels, true, false).unwrap();

    assert_eq!(
        out.data,
        vec!["A", "A", "B", "B", "C"]
    );
    assert_eq!(out.validity.to_vec(), bitvec![1,1,1,1,1]);
}

#[test]
fn test_i64_cut_labels_bounded_left() {
    let v = vd_i64(vec![-2, -1, 0, 1, 2], vec![true; 5]);
    let bins = vec![-1.0, 1.0];
    let labels = vec!["MID"].into_iter().map(String::from).collect::<Vec<_>>();

    let out = v.cut_labels(&bins, &labels, false, true).unwrap();

    assert_eq!(
        out.data,
        vec!["MID", "MID", "MID", "MID", "MID"]
    );
    assert_eq!(out.validity.to_vec(), bitvec![0,1,1,0,0]);
}

#[test]
fn test_cut_labels_invalid_label_len_bounded() {
    let v = vd_f64(vec![0.0], vec![true]);
    let bins = vec![-1.0, 1.0];
    let labels = vec!["A".to_string(), "B".to_string()];

    let err = v.cut_labels(&bins, &labels, true, true).unwrap_err();

    matches!(err, ErebusError::InvalidCutLabels { .. });
}

#[test]
fn test_cut_labels_invalid_label_len_unbounded() {
    let v = vd_f64(vec![0.0], vec![true]);
    let bins = vec![-1.0, 1.0];
    let labels = vec!["A".to_string()];

    let err = v.cut_labels(&bins, &labels, true, false).unwrap_err();

    matches!(err, ErebusError::InvalidCutLabels { .. });
}

#[test]
fn test_cut_labels_non_monotonic_bins() {
    let v = vd_f64(vec![0.0], vec![true]);
    let bins = vec![1.0, -1.0];
    let labels = vec!["A".to_string(), "B".to_string(), "C".to_string()];

    let err = v.cut_labels(&bins, &labels, true, false).unwrap_err();

    matches!(err, ErebusError::InvalidCutBins { .. });
}

#[test]
fn test_f64_reciprocal_owned() {
    let input = vec![2.0, -4.0, 0.0, 0.5];
    let validity = vec![true; 4];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.reciprocal();

    assert_vd_f64_matches(&out, &input, &validity, |x| 1.0 / x);
}

#[test]
fn test_f64_reciprocal_inplace() {
    let input = vec![1.0, 2.0, 0.0, -0.5];
    let validity = vec![true; 4];

    let mut v = vd_f64(input.clone(), validity.clone());
    v.reciprocal_inplace();

    assert_vd_f64_matches(&v, &input, &validity, |x| 1.0 / x);
}

#[test]
fn test_f64_reciprocal_range_full() {
    let input = vec![1.0, 2.0, 0.0, 4.0];
    let validity = vec![true; 4];

    let v = vd_f64(input.clone(), validity.clone());
    let out = v.reciprocal_range(1, 3, true);

    assert_vd_f64_matches_range(&out, &input, &validity, 1, 3, |x| 1.0 / x);
}

#[test]
fn test_i64_reciprocal_owned() {
    let input = vec![1, 2, 0, -4];
    let validity = vec![true; 4];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.reciprocal();

    assert_vd_f64_matches(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        |x| 1.0 / x
    );
}

#[test]
fn test_i64_reciprocal_range() {
    let input = vec![1, 2, 0, -4];
    let validity = vec![true; 4];

    let v = vd_i64(input.clone(), validity.clone());
    let out = v.reciprocal_range(1, 4, true);

    assert_vd_f64_matches_range(
        &out,
        &input.iter().map(|&x| x as f64).collect::<Vec<_>>(),
        &validity,
        1,
        4,
        |x| 1.0 / x
    );
}

#[test]
fn test_i64_cmp_owned() {
    let v = vd_i64(vec![1, 2, 3, 4], vec![true, false, true, true]);

    let lt = v.lt(3);
    assert_eq!(lt.data, vec![true, true, false, false]);
    assert_eq!(lt.validity.to_vec(), bitvec![1, 0, 1, 1]);

    let lte = v.lte(3);
    assert_eq!(lte.data, vec![true, true, true, false]);

    let gt = v.gt(3);
    assert_eq!(gt.data, vec![false, false, false, true]);

    let gte = v.gte(3);
    assert_eq!(gte.data, vec![false, false, true, true]);
}

#[test]
fn test_i64_cmp_range_full() {
    let v = vd_i64(vec![1, 2, 3, 4], vec![true; 4]);

    let out = v.gt_range(2, 1, 3, true);

    assert_eq!(out.data, vec![false, false, true, false]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1, 1, 0]);
}

#[test]
fn test_i64_cmp_range_slice_only() {
    let v = vd_i64(vec![1, 2, 3, 4], vec![true; 4]);

    let out = v.lte_range(2, 1, 3, false);

    assert_eq!(out.data, vec![true, false]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_f64_cmp_owned() {
    let v = vd_f64(vec![1.0, 2.5, 3.0, 4.0], vec![true, false, true, true]);

    let lt = v.lt(3.0);
    assert_eq!(lt.data, vec![true, true, false, false]);
    assert_eq!(lt.validity.to_vec(), bitvec![1, 0, 1, 1]);

    let lte = v.lte(3.0);
    assert_eq!(lte.data, vec![true, true, true, false]);

    let gt = v.gt(3.0);
    assert_eq!(gt.data, vec![false, false, false, true]);

    let gte = v.gte(3.0);
    assert_eq!(gte.data, vec![false, false, true, true]);
}

#[test]
fn test_f64_cmp_range_full() {
    let v = vd_f64(vec![1.0, 2.0, 3.0, 4.0], vec![true; 4]);

    let out = v.lt_range(3.0, 1, 3, true);

    assert_eq!(out.data, vec![false, true, false, false]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1, 1, 0]);
}

#[test]
fn test_f64_cmp_range_slice_only() {
    let v = vd_f64(vec![1.0, 2.0, 3.0, 4.0], vec![true; 4]);

    let out = v.gte_range(2.0, 1, 3, false);

    assert_eq!(out.data, vec![true, true]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_i64_threshold_owned() {
    let v = vd_i64(vec![1, 3, 5, 7], vec![true, false, true, true]);

    let out = v.threshold(4.0);

    assert_eq!(out.data, vec![false, false, true, true]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1, 1]);
}

#[test]
fn test_i64_threshold_range_full() {
    let v = vd_i64(vec![1, 3, 5, 7], vec![true; 4]);

    let out = v.threshold_range(4.0, 1, 3, true);

    assert_eq!(out.data, vec![false, false, true, false]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1, 1, 0]);
}

#[test]
fn test_i64_between_owned() {
    let v = vd_i64(vec![1, 3, 5, 7], vec![true; 4]);

    let out = v.between(3, 6);

    assert_eq!(out.data, vec![false, true, true, false]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_f64_threshold_owned() {
    let v = vd_f64(vec![1.0, 2.5, 5.0, 7.0], vec![true, false, true, true]);

    let out = v.threshold(3.0);

    assert_eq!(out.data, vec![false, false, true, true]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1, 1]);
}

#[test]
fn test_f64_threshold_eq_range_full() {
    let v = vd_f64(vec![1.0, 2.5, 5.0, 7.0], vec![true; 4]);

    let out = v.threshold_eq_range(5.0, 1, 3, true);

    assert_eq!(out.data, vec![false, false, true, false]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1, 1, 0]);
}

#[test]
fn test_f64_between_owned() {
    let v = vd_f64(vec![1.0, 2.5, 5.0, 7.0], vec![true; 4]);

    let out = v.between(2.0, 6.0);

    assert_eq!(out.data, vec![false, true, true, false]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_f64_binarize_owned() {
    let v = vd_f64(vec![1.0, 5.0, 10.0, -2.0], vec![true, true, false, true]);

    let out = v.binarize(3.0);

    assert_eq!(out.data, vec![0, 1, 1, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 0, 1]);
}

#[test]
fn test_f64_binarize_eq_owned() {
    let v = vd_f64(vec![3.0, 5.0, 3.0], vec![true; 3]);

    let out = v.binarize_eq(3.0);

    assert_eq!(out.data, vec![1, 1, 1]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1]);
}

#[test]
fn test_f64_binarize_range_slice_only() {
    let v = vd_f64(vec![1.0, 5.0, 10.0, -2.0], vec![true; 4]);

    let out = v.binarize_range(3.0, 1, 3, false);

    assert_eq!(out.data, vec![1, 1]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_f64_binarize_range_full() {
    let v = vd_f64(vec![1.0, 5.0, 10.0, -2.0], vec![true; 4]);

    let out = v.binarize_range(3.0, 1, 3, true);

    assert_eq!(out.data, vec![0, 1, 1, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1, 1, 0]);
}

#[test]
fn test_f64_binarize_eq_range_full_with_nulls() {
    let v = vd_f64(vec![3.0, 2.0, 3.0, 4.0], vec![true, false, true, true]);

    let out = v.binarize_eq_range(3.0, 1, 4, true);

    assert_eq!(out.data, vec![0, 0, 1, 1]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 0, 1, 1]);
}

#[test]
fn test_i64_binarize_owned() {
    let v = vd_i64(vec![1, 5, 10, -2], vec![true, true, false, true]);

    let out = v.binarize(3.0);

    assert_eq!(out.data, vec![0, 1, 1, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 0, 1]);
}

#[test]
fn test_i64_binarize_eq_owned() {
    let v = vd_i64(vec![3, 5, 10, 3], vec![true; 4]);

    let out = v.binarize_eq(3.0);

    assert_eq!(out.data, vec![1, 1, 1, 1]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1, 1, 1]);
}

#[test]
fn test_i64_binarize_range_slice_only() {
    let v = vd_i64(vec![1, 5, 10, -2], vec![true; 4]);

    let out = v.binarize_range(3.0, 1, 3, false);

    assert_eq!(out.data, vec![1, 1]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_i64_binarize_range_full() {
    let v = vd_i64(vec![1, 5, 10, -2], vec![true; 4]);

    let out = v.binarize_range(3.0, 1, 3, true);

    assert_eq!(out.data, vec![0, 1, 1, 0]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 1, 1, 0]);
}

#[test]
fn test_i64_binarize_eq_range_full_with_nulls() {
    let v = vd_i64(vec![3, 2, 3, 4], vec![true, false, true, true]);

    let out = v.binarize_eq_range(3.0, 1, 4, true);

    assert_eq!(out.data, vec![0, 0, 1, 1]);
    assert_eq!(out.validity.to_vec(), bitvec![0, 0, 1, 1]);
}