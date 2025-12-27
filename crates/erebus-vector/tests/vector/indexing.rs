// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

fn v_i64(v: Vec<i64>) -> Vector<i64> { Vector { data: v } }
fn v_f64(v: Vec<f64>) -> Vector<f64> { Vector { data: v } }
fn v_bool(v: Vec<bool>) -> Vector<bool> { Vector { data: v } }

#[test]
fn test_first_basic() {
    let v = v_i64(vec![10, 20, 30]);
    assert_eq!(v.first(), Some(10));

    let empty = v_i64(vec![]);
    assert_eq!(empty.first(), None);
}

#[test]
fn test_last_basic() {
    let v = v_i64(vec![10, 20, 30]);
    assert_eq!(v.last(), Some(30));

    let empty = v_i64(vec![]);
    assert_eq!(empty.last(), None);
}

#[test]
fn test_get_basic() {
    let v = v_i64(vec![5, 6, 7]);

    assert_eq!(v.get(0), Some(5));
    assert_eq!(v.get(2), Some(7));

    assert_eq!(v.get(3), None); // OOB
}

#[test]
fn test_nth_basic() {
    let v = v_i64(vec![10, 20, 30, 40]);

    assert_eq!(v.nth(0), Some(10));
    assert_eq!(v.nth(2), Some(30));
    assert_eq!(v.nth(3), Some(40));

    // negative indexing
    assert_eq!(v.nth(-1), Some(40));
    assert_eq!(v.nth(-2), Some(30));
    assert_eq!(v.nth(-4), Some(10));

    // OOB negative
    assert_eq!(v.nth(-5), None);

    // OOB positive
    assert_eq!(v.nth(10), None);
}

#[test]
fn test_take_basic() {
    let v = v_i64(vec![10, 20, 30, 40]);

    let out = v.take(&[0, 3, 1]).unwrap();
    assert_eq!(out.data, vec![10, 40, 20]);
}

#[test]
fn test_take_oob_error() {
    let v = v_i64(vec![10, 20, 30]);

    let err = v.take(&[0, 5, 1]).unwrap_err();

    match err {
        ErebusError::IndexOutOfBounds { index, size } => {
            assert_eq!(index, 5);
            assert_eq!(size, 3);
        }
        other => panic!("Expected IndexOutOfBounds error, got {:?}", other),
    }
}

#[test]
fn test_bool_index_basic() {
    let v = v_i64(vec![10, 20, 30, 40]);
    let mask = vec![false, true, false, true];

    let out = v.bool_index(&mask).unwrap();

    assert_eq!(out.data, vec![20, 40]);
}

#[test]
fn test_bool_index_all_true() {
    let v = v_i64(vec![1, 2, 3]);
    let mask = vec![true, true, true];

    let out = v.bool_index(&mask).unwrap();
    assert_eq!(out.data, vec![1, 2, 3]);
}

#[test]
fn test_bool_index_mismatch_error() {
    let v = v_i64(vec![10, 20, 30]);
    let bad_mask = vec![true, false]; // too short

    let err = v.bool_index(&bad_mask).unwrap_err();

    match err {
        ErebusError::LengthMismatch { expected, found } => {
            assert_eq!(expected, 3);
            assert_eq!(found, 2);
        }
        other => panic!("Expected LengthMismatch, got {:?}", other),
    }
}

//
// Boolean vector extras: arg_true / arg_false
//

#[test]
fn test_arg_true() {
    let v = v_bool(vec![true, false, true, false]);
    assert_eq!(v.arg_true(), vec![0, 2]);
}

#[test]
fn test_arg_false() {
    let v = v_bool(vec![true, false, false, true]);
    assert_eq!(v.arg_false(), vec![1, 2]);
}

#[test]
fn test_arg_true_empty() {
    let v = v_bool(vec![false, false, false]);
    assert_eq!(v.arg_true(), Vec::<usize>::new());
}

#[test]
fn test_arg_false_empty() {
    let v = v_bool(vec![true, true, true]);
    assert_eq!(v.arg_false(), Vec::<usize>::new());
}

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