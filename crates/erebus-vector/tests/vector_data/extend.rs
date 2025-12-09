// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

#[test]
fn test_extend_from_slice() {
    let mut v = VectorData::from_vec(vec![1, 2, 3], bitvec![1, 1, 1]).unwrap();
    v.extend(&[4, 5]);
    assert_eq!(v.data, vec![1, 2, 3, 4, 5]);
    assert!(v.validity.all());
}

#[test]
fn test_append_from_other() {
    let mut v1 = VectorData::full(1, 3);
    let v2 = VectorData::full(2, 2);
    v1.append(&v2);
    assert_eq!(v1.data, vec![1, 1, 1, 2, 2]);
    assert!(v1.validity.all());
}

#[test]
fn test_concat_creates_new() {
    let v1 = VectorData::full(10, 2);
    let v2 = VectorData::full(20, 3);
    let v3 = v1.concat(&v2);
    assert_eq!(v3.data, vec![10, 10, 20, 20, 20]);
    assert!(v3.validity.all());
}

#[test]
fn test_stack_multiple() {
    let v1 = VectorData::full(1, 2);
    let v2 = VectorData::full(2, 2);
    let v3 = VectorData::full(3, 2);
    let stacked = VectorData::stack(&[&v1, &v2, &v3]);
    assert_eq!(stacked.data, vec![1, 1, 2, 2, 3, 3]);
    assert!(stacked.validity.all());
}