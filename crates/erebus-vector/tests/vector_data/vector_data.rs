// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

#[test]
fn test_vector_data_empty() {
    let v: VectorData<i32> = VectorData::empty();
    assert_eq!(v.len(), 0);
    assert_eq!(v.null_count(), 0);
    assert!(v.validity.is_empty());
}

#[test]
fn test_vector_data_new() {
    let v: VectorData<i32> = VectorData::new();
    assert_eq!(v.len(), 0);
    assert_eq!(v.null_count(), 0);
    assert!(v.validity.is_empty());
}

#[test]
fn test_vector_data_full() {
    let v = VectorData::full(10, 5);
    assert_eq!(v.len(), 5);
    assert_eq!(v.null_count(), 0);
    assert!(v.validity.all());
    assert!(v.data.iter().all(|&x| x == 10));
}

#[test]
fn test_vector_data_print_with_nulls() {
    let data = vec![10, 20, 30];
    let validity = bitvec![1, 0, 1];
    let v = VectorData::from_vec(data, validity).unwrap();
    let printed = v.print();
    assert_eq!(printed, "[10, None, 30]");
}

#[test]
fn test_vector_data_memory_usage() {
    let v = VectorData::full(1, 10);
    assert!(v.memory_usage() > 0);
}

#[test]
fn test_from_vec_safe() {
    let data = vec![1, 2, 3];
    let validity = bitvec![1, 0, 1];
    let v = VectorData::from_vec(data.clone(), validity.clone()).unwrap();
    assert_eq!(v.data, data);
    assert_eq!(v.validity, validity);
}

#[test]
fn test_from_vec_length_mismatch() {
    let data = vec![1, 2, 3];
    let validity = bitvec![1, 1]; // mismatch
    let err = VectorData::from_vec(data, validity).unwrap_err();
    if let ErebusError::VectorLengthMismatch { expected, found } = err {
        assert_eq!(expected, 3);
        assert_eq!(found, 2);
    } else {
        panic!("Unexpected error type");
    }
}

#[test]
fn test_internal_from_vec() {
    let data = vec![1, 2, 3];
    let validity = bitvec![1, 0, 1];
    let v = VectorData::_from_vec(data.clone(), validity.clone());
    assert_eq!(v.data, data);
    assert_eq!(v.validity, validity);
}