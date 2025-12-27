// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

#[test]
fn test_vector_slice_basic() {
    let v = Vector::from_vec(vec![10, 20, 30, 40, 50]).unwrap();
    let sliced = v.slice(1, 4);
    assert_eq!(sliced.data, vec![20, 30, 40]);
}

#[test]
fn test_vector_slice_entire_range() {
    let v = Vector::from_vec(vec![1, 2, 3]).unwrap();
    let sliced = v.slice(0, 3);
    assert_eq!(sliced.data, vec![1, 2, 3]);
}

#[test]
fn test_vector_slice_out_of_bounds() {
    let v = Vector::from_vec(vec![1, 2, 3]).unwrap();
    let sliced = v.slice(2, 10);
    assert_eq!(sliced.data, vec![3]); // clamps end = len
}

#[test]
fn test_vector_slice_invalid_range_returns_empty() {
    let v = Vector::from_vec(vec![1, 2, 3]).unwrap();
    let sliced = v.slice(3, 2);
    assert_eq!(sliced.data.len(), 0);
}

#[test]
fn test_vector_slice_view_borrow() {
    let v = Vector::from_vec(vec![100, 200, 300, 400]).unwrap();
    let view = v.slice_view(1, 3);
    assert_eq!(view.data, &[200, 300]);

    // Borrowed view should reflect original memory
    assert_eq!(view.data.len(), 2);
    assert_eq!(v.data[1], 200);
}

#[test]
fn test_vector_slice_view_out_of_bounds_returns_empty() {
    let v = Vector::from_vec(vec![10, 20]).unwrap();
    let view = v.slice_view(5, 8);
    assert!(view.data.is_empty());
}