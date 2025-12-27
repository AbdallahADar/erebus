// === Imports ===
use erebus_time::prelude::*;

// === Tests ===

fn sample_date_vector() -> DateVector {
    // Index: 0        1        2        3
    // Date:  2024-03-15  invalid  2024-02-29  invalid
    DateVector::from_ymd_vectors(
        &[2024, 2023, 2024, 2022],
        &[3,    2,    2,    13],
        &[15,   29,   29,   10],
    ).unwrap()
}

#[test]
fn test_date_vector_get_valid() {
    let v = sample_date_vector();
    let d = v.get(0).unwrap();
    assert_eq!(d.ymd(), (2024, 3, 15));
}

#[test]
fn test_date_vector_get_null() {
    let v = sample_date_vector();
    assert!(v.get(1).is_none());
}

#[test]
fn test_date_vector_get_oob() {
    let v = sample_date_vector();
    assert!(v.get(10).is_none());
}

#[test]
fn test_date_vector_take_mixed() {
    let v = sample_date_vector();
    let out = v.take(&[0, 1, 2]).unwrap();

    assert_eq!(out.len(), 3);

    assert_eq!(out.get(0).unwrap().ymd(), (2024, 3, 15));
    assert!(out.get(1).is_none());
    assert_eq!(out.get(2).unwrap().ymd(), (2024, 2, 29));
}

#[test]
fn test_date_vector_take_oob() {
    let v = sample_date_vector();
    let err = v.take(&[0, 5]).unwrap_err();

    match err {
        ErebusError::IndexOutOfBounds { index, size } => {
            assert_eq!(index, 5);
            assert_eq!(size, 4);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_vector_bool_index() {
    let v = sample_date_vector();

    let mask = [true, false, true, false];
    let out = v.bool_index(&mask).unwrap();

    assert_eq!(out.len(), 2);

    assert_eq!(out.get(0).unwrap().ymd(), (2024, 3, 15));
    assert_eq!(out.get(1).unwrap().ymd(), (2024, 2, 29));
}

#[test]
fn test_date_vector_bool_index_len_mismatch() {
    let v = sample_date_vector();
    let mask = [true, false];

    let err = v.bool_index(&mask).unwrap_err();

    match err {
        ErebusError::LengthMismatch { expected, found } => {
            assert_eq!(expected, 4);
            assert_eq!(found, 2);
        }
        other => panic!("Unexpected error: {:?}", other),
    }
}

#[test]
fn test_date_vector_slice_normal() {
    let v = sample_date_vector();
    let out = v.slice(1, 4);

    assert_eq!(out.len(), 3);

    assert!(out.get(0).is_none());
    assert_eq!(out.get(1).unwrap().ymd(), (2024, 2, 29));
    assert!(out.get(2).is_none());
}

#[test]
fn test_date_vector_slice_empty_range() {
    let v = sample_date_vector();
    let out = v.slice(3, 2);

    assert!(out.is_empty());
}

#[test]
fn test_date_vector_slice_oob_start() {
    let v = sample_date_vector();
    let out = v.slice(10, 20);

    assert!(out.is_empty());
}