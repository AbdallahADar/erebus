// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

#[test]
fn test_vector_empty() {
    let v: Vector<i32> = Vector::empty();
    assert_eq!(v.len(), 0);
    assert!(v.data.is_empty());
}

#[test]
fn test_vector_new() {
    let v: Vector<i32> = Vector::new();
    assert_eq!(v.len(), 0);
    assert!(v.data.is_empty());
}

#[test]
fn test_vector_full() {
    let v = Vector::full(7, 3);
    assert_eq!(v.len(), 3);
    assert_eq!(v.data, vec![7, 7, 7]);
}

#[test]
fn test_vector_len_and_memory_usage() {
    let v = Vector::full(5, 10);
    assert_eq!(v.len(), 10);
    assert!(v.memory_usage() > 0);
}

#[test]
fn test_vector_print() {
    let v = Vector::full(3, 4);
    let output = v.print();
    assert_eq!(output, "[3, 3, 3, 3]");
}

#[test]
fn test_vector_from_vec_safe() {
    let data = vec![1, 2, 3];
    let v = Vector::from_vec(data.clone()).unwrap();
    assert_eq!(v.data, data);
}

#[test]
fn test_vector_from_vec_empty_error() {
    let data: Vec<i32> = vec![];
    let err = Vector::from_vec(data).unwrap_err();
    if let ErebusError::EmptyVector = err {
        // expected
    } else {
        panic!("Unexpected error type");
    }
}

#[test]
fn test_vector_internal_from_vec() {
    let data = vec![10, 20, 30];
    let v = Vector::_from_vec(data.clone());
    assert_eq!(v.data, data);
}