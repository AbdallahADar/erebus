// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

#[test]
fn test_is_valid_at_safe() {
    let v = VectorData::from_vec(vec![10, 20, 30], bitvec![1, 0, 1]).unwrap();
    assert_eq!(v.is_valid_at(0).unwrap(), true);
    assert_eq!(v.is_valid_at(1).unwrap(), false);
    assert_eq!(v.is_valid_at(2).unwrap(), true);
}

#[test]
fn test_is_valid_at_out_of_bounds_error() {
    let v = VectorData::from_vec(vec![10, 20], bitvec![1, 0]).unwrap();
    let result = v.is_valid_at(3);
    assert!(result.is_err());
    if let Err(ErebusError::IndexOutOfBounds { index, size }) = result {
        assert_eq!(index, 3);
        assert_eq!(size, 2);
    } else {
        panic!("Unexpected error type");
    }
}

#[test]
fn test_with_validity_safe() {
    let v = VectorData::from_vec(vec![1, 2, 3], bitvec![1, 1, 1]).unwrap();
    let new_mask = bitvec![1, 0, 0];
    let v2 = v.with_validity(new_mask.clone()).unwrap();
    assert_eq!(v2.validity, new_mask);
}

#[test]
fn test_set_validity_safe() {
    let mut v = VectorData::from_vec(vec![1, 2, 3], bitvec![1, 1, 1]).unwrap();
    let new_mask = bitvec![0, 1, 0];
    v.set_validity(new_mask.clone()).unwrap();
    assert_eq!(v.validity, new_mask);
}

#[test]
fn test_with_validity_length_mismatch_error() {
    let v = VectorData::from_vec(vec![1, 2, 3], bitvec![1, 1, 1]).unwrap();
    let invalid_mask = bitvec![1, 0]; // shorter
    let result = v.with_validity(invalid_mask);
    assert!(result.is_err());
}

#[test]
fn test_set_validity_length_mismatch_error() {
    let mut v = VectorData::from_vec(vec![1, 2, 3], bitvec![1, 1, 1]).unwrap();
    let invalid_mask = bitvec![1, 0];
    let result = v.set_validity(invalid_mask);
    assert!(result.is_err());
}

#[test]
fn test_internal_is_valid_at_fast() {
    let v = VectorData::from_vec(vec![10, 20, 30], bitvec![1, 0, 1]).unwrap();
    // direct internal access
    assert_eq!(v._is_valid_at(1), false);
    assert_eq!(v._is_valid_at(2), true);
}

#[test]
fn test_internal_set_validity_fast() {
    let mut v = VectorData::from_vec(vec![1, 2, 3], bitvec![1, 1, 1]).unwrap();
    let new_mask = bitvec![1, 0, 1];
    v._set_validity(new_mask.clone());
    assert_eq!(v.validity, new_mask);
}

#[test]
fn test_internal_with_validity_fast() {
    let v = VectorData::from_vec(vec![1, 2, 3], bitvec![1, 1, 1]).unwrap();
    let new_mask = bitvec![0, 1, 0];
    let v2 = v._with_validity(new_mask.clone());
    assert_eq!(v2.validity, new_mask);
}