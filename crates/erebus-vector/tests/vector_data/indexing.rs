// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

// Helper constructors
fn vd_i64(data: Vec<i64>, valid: Vec<bool>) -> VectorData<i64> {
    VectorData::from_vec(data, BitVec::from_iter(valid)).unwrap()
}

fn vd_str(data: Vec<&str>, valid: Vec<bool>) -> VectorData<String> {
    VectorData::from_vec(
        data.into_iter().map(|s| s.to_string()).collect(),
        BitVec::from_iter(valid),
    )
    .unwrap()
}

fn vd_bool(data: Vec<bool>, valid: Vec<bool>) -> VectorData<bool> {
    VectorData::from_vec(data, BitVec::from_iter(valid)).unwrap()
}

#[test]
fn test_first_and_last() {
    let v = vd_i64(vec![10, 20, 30], vec![true, true, true]);

    assert_eq!(v.first(), Some(10));
    assert_eq!(v.last(), Some(30));

    let v2 = vd_i64(vec![10, 20, 30], vec![false, true, false]);

    assert_eq!(v2.first(), None);
    assert_eq!(v2.last(), None);
}

#[test]
fn test_first_valid_and_last_valid() {
    let v = vd_i64(vec![10, 20, 30, 40], vec![false, false, true, false]);

    assert_eq!(v.first_valid(), Some(30));
    assert_eq!(v.last_valid(), Some(30));

    let v2 = vd_i64(vec![1, 2, 3], vec![false, false, false]);
    assert_eq!(v2.first_valid(), None);
    assert_eq!(v2.last_valid(), None);
}

#[test]
fn test_get_basic() {
    let v = vd_i64(vec![5, 10, 15], vec![true, false, true]);

    assert_eq!(v.get(0), Some(5));
    assert_eq!(v.get(1), None);  // invalid
    assert_eq!(v.get(2), Some(15));

    assert_eq!(v.get(999), None); // OOB
}

#[test]
fn test_nth_positive_and_negative() {
    let v = vd_i64(vec![1, 2, 3, 4], vec![true, true, false, true]);

    assert_eq!(v.nth(0), Some(1));
    assert_eq!(v.nth(1), Some(2));
    assert_eq!(v.nth(2), None);        // null
    assert_eq!(v.nth(3), Some(4));
    assert_eq!(v.nth(4), None);        // OOB

    // negative indexing
    assert_eq!(v.nth(-1), Some(4));
    assert_eq!(v.nth(-2), None);       // null element index 2
    assert_eq!(v.nth(-4), Some(1));
    assert_eq!(v.nth(-5), None);       // OOB
}

#[test]
fn test_bool_index_basic() {
    let vd = vd_i64(vec![10, 20, 30, 40], vec![true, true, true, true]);
    let mask = vec![false, true, false, true];

    let out = vd.bool_index(&mask).unwrap();

    assert_eq!(out.data, vec![20, 40]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 1]);
}

#[test]
fn test_bool_index_preserves_nulls() {
    let vd = vd_i64(
        vec![10, 20, 30, 40],
        vec![true, false, true, false],
    );
    let mask = vec![true, true, true, true];

    let out = vd.bool_index(&mask).unwrap();

    assert_eq!(out.data, vec![10, 20, 30, 40]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1,0]);
}

#[test]
fn test_bool_index_all_false() {
    let vd = vd_i64(vec![10, 20, 30], vec![true, true, true]);
    let mask = vec![false, false, false];

    let out = vd.bool_index(&mask).unwrap();

    assert_eq!(out.data.len(), 0);
    assert_eq!(out.validity.len(), 0);
}

#[test]
fn test_bool_index_all_true() {
    let vd = vd_i64(vec![7, 8, 9], vec![true, false, true]);
    let mask = vec![true, true, true];

    let out = vd.bool_index(&mask).unwrap();

    assert_eq!(out.data, vec![7, 8, 9]);
    assert_eq!(out.validity.to_vec(), bitvec![1, 0, 1]);
}

#[test]
fn test_bool_index_mismatched_length() {
    let vd = vd_i64(vec![1, 2, 3], vec![true, true, true]);
    let mask = vec![true, true]; // wrong length

    let result = vd.bool_index(&mask);

    assert!(result.is_err());
    if let Err(ErebusError::VectorLengthMismatch { expected, found }) = result {
        assert_eq!(expected, 3);
        assert_eq!(found, 2);
    } else {
        panic!("Expected VectorLengthMismatch error");
    }
}

#[test]
fn test_arg_true() {
    let vd = vd_bool(vec![true, false, true, false], vec![true; 4]);
    assert_eq!(vd.arg_true(), vec![0, 2]);
}

#[test]
fn test_arg_false() {
    let vd = vd_bool(vec![true, false, true, false], vec![true; 4]);
    assert_eq!(vd.arg_false(), vec![1, 3]);
}

#[test]
fn test_take_valid() {
    let v = vd_i64(vec![5, 10, 15, 20], vec![true, false, true, true]);

    let out = v.take(&[0, 2, 3]).unwrap();
    assert_eq!(out.data, vec![5, 15, 20]);
    assert_eq!(out.validity, bitvec![1, 1, 1]);
}

#[test]
fn test_take_oob() {
    let v = vd_i64(vec![1, 2, 3], vec![true; 3]);

    let res = v.take(&[0, 10]);
    assert!(res.is_err());
}

#[test]
fn test_internal_get() {
    let v = vd_i64(vec![100, 200], vec![true, false]);

    assert_eq!(v._get(0), Some(100));
    assert_eq!(v._get(1), None);
}

#[test]
fn test_internal_take() {
    let v = vd_i64(vec![1, 2, 3], vec![true, true, false]);

    let out = v._take(&[0, 2]);
    assert_eq!(out.data, vec![1, 3]);
    assert_eq!(out.validity, bitvec![1, 0]);
}

#[test]
fn test_vector_data_slice_basic() {
    let v = VectorData::from_vec(
        vec![10, 20, 30, 40, 50],
        bitvec![1, 1, 0, 1, 1],
    )
    .unwrap();
    let sliced = v.slice(1, 4);
    assert_eq!(sliced.data, vec![20, 30, 40]);
    assert_eq!(sliced.validity, bitvec![1, 0, 1]);
}

#[test]
fn test_vector_data_slice_entire_range() {
    let v = VectorData::from_vec(vec![1, 2, 3], bitvec![1, 1, 1]).unwrap();
    let sliced = v.slice(0, 3);
    assert_eq!(sliced.data, vec![1, 2, 3]);
    assert_eq!(sliced.validity, bitvec![1, 1, 1]);
}

#[test]
fn test_vector_data_slice_out_of_bounds() {
    let v = VectorData::from_vec(vec![1, 2, 3], bitvec![1, 1, 1]).unwrap();
    let sliced = v.slice(2, 10);
    assert_eq!(sliced.data, vec![3]);
    assert_eq!(sliced.validity, bitvec![1]);
}

#[test]
fn test_vector_data_slice_invalid_range_returns_empty() {
    let v = VectorData::from_vec(vec![1, 2, 3], bitvec![1, 1, 1]).unwrap();
    let sliced = v.slice(3, 2);
    assert_eq!(sliced.data.len(), 0);
    assert!(sliced.validity.is_empty());
}

#[test]
fn test_vector_data_slice_view_borrow() {
    let v = VectorData::from_vec(
        vec![10, 20, 30, 40],
        bitvec![1, 0, 1, 1],
    )
    .unwrap();
    let view = v.slice_view(1, 3);
    assert_eq!(view.data, &[20, 30]);
    assert_eq!(view.validity.len(), 2);
    assert!(!view.validity[0]); // 20 invalid
    assert!(view.validity[1]);  // 30 valid
}

#[test]
fn test_vector_data_slice_view_out_of_bounds_returns_empty() {
    let v = VectorData::from_vec(vec![10, 20], bitvec![1, 1]).unwrap();
    let view = v.slice_view(5, 8);
    assert!(view.data.is_empty());
    assert!(view.validity.is_empty());
}