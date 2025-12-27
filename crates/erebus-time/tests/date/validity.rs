// === Imports ===
use erebus_time::prelude::*;

// === Tests ===

fn sample_date_vector() -> DateVector {
    DateVector::from_ymd_vectors(
        &[2024, 2023, 2024, 2022],
        &[3,    2,    2,    13],
        &[15,   29,   29,   10],
    ).unwrap()
}

#[test]
fn test_date_vector_is_valid_at_true() {
    let v = sample_date_vector();
    assert_eq!(v.is_valid_at(0).unwrap(), true);
}

#[test]
fn test_date_vector_is_valid_at_false() {
    let v = sample_date_vector();
    assert_eq!(v.is_valid_at(1).unwrap(), false);
}

#[test]
fn test_date_vector_is_valid_at_oob() {
    let v = sample_date_vector();
    let err = v.is_valid_at(10).unwrap_err();

    match err {
        ErebusError::IndexOutOfBounds { index, size } => {
            assert_eq!(index, 10);
            assert_eq!(size, 4);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_vector_set_validity_ok() {
    let mut v = sample_date_vector();

    let new_validity = bitvec![1, 1, 0, 0];
    v.set_validity(new_validity).unwrap();

    assert_eq!(v.is_valid_at(0).unwrap(), true);
    assert_eq!(v.is_valid_at(1).unwrap(), true);
    assert_eq!(v.is_valid_at(2).unwrap(), false);
    assert_eq!(v.is_valid_at(3).unwrap(), false);
}

#[test]
fn test_date_vector_set_validity_len_mismatch() {
    let mut v = sample_date_vector();

    let bad_validity = bitvec![1, 0];
    let err = v.set_validity(bad_validity).unwrap_err();

    match err {
        ErebusError::LengthMismatch { expected, found } => {
            assert_eq!(expected, 4);
            assert_eq!(found, 2);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_vector_with_validity_ok() {
    let v = sample_date_vector();

    let new_validity = bitvec![0, 1, 1, 0];
    let out = v.with_validity(new_validity).unwrap();

    assert_eq!(out.is_valid_at(0).unwrap(), false);
    assert_eq!(out.is_valid_at(1).unwrap(), true);
    assert_eq!(out.is_valid_at(2).unwrap(), true);
    assert_eq!(out.is_valid_at(3).unwrap(), false);
}

#[test]
fn test_date_vector_with_validity_len_mismatch() {
    let v = sample_date_vector();

    let bad_validity = bitvec![1, 0, 1];
    let err = v.with_validity(bad_validity).unwrap_err();

    match err {
        ErebusError::LengthMismatch { expected, found } => {
            assert_eq!(expected, 4);
            assert_eq!(found, 3);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}