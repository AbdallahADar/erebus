// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

#[test]
fn test_vector_extend_from_slice() {
    let mut v = Vector::from_vec(vec![1, 2, 3]).unwrap();
    v.extend(&[4, 5]);
    assert_eq!(v.data, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_vector_append_and_concat() {
    let mut v1 = Vector::from_vec(vec![1, 2]).unwrap();
    let v2 = Vector::from_vec(vec![3, 4]).unwrap();
    v1.append(&v2);
    assert_eq!(v1.data, vec![1, 2, 3, 4]);

    let v3 = Vector::from_vec(vec![5, 6]).unwrap();
    let result = v1.concat(&v3);
    assert_eq!(result.data, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_vector_stack() {
    let v1 = Vector::from_vec(vec![1, 2]).unwrap();
    let v2 = Vector::from_vec(vec![3, 4]).unwrap();
    let v3 = Vector::from_vec(vec![5, 6]).unwrap();

    let stacked = Vector::stack(&[&v1, &v2, &v3]);
    assert_eq!(stacked.data, vec![1, 2, 3, 4, 5, 6]);
}