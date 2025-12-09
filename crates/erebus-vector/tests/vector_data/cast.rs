// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

#[test]
fn test_is_numeric_i64_and_f64() {
    let vi = VectorData::full(1_i64, 3);
    let vf = VectorData::full(1.0_f64, 3);
    let vb = VectorData::full(true, 3);
    let vs = VectorData::full("a".to_string(), 3);

    assert!(vi.is_numeric());
    assert!(vf.is_numeric());
    assert!(!vb.is_numeric());
    assert!(!vs.is_numeric());
}

// ------------------ i64 CASTS ------------------

#[test]
fn test_i64_to_float() {
    let v = VectorData::from_vec(vec![1_i64, 2, 3], bitvec![1, 1, 1]).unwrap();
    let vf = v.to_float();
    assert_eq!(vf.data, vec![1.0, 2.0, 3.0]);
    assert_eq!(vf.validity, bitvec![1, 1, 1]);
}

#[test]
fn test_i64_to_bool() {
    let v = VectorData::from_vec(vec![0_i64, 2, 0, 5], bitvec![1, 1, 1, 1]).unwrap();
    let vb = v.to_bool();
    assert_eq!(vb.data, vec![false, true, false, true]);
}

#[test]
fn test_i64_to_text() {
    let v = VectorData::from_vec(vec![10_i64, -2, 0], bitvec![1, 1, 1]).unwrap();
    let vs = v.to_text();
    assert_eq!(vs.data, vec!["10".to_string(), "-2".to_string(), "0".to_string()]);
}

// ------------------ f64 CASTS ------------------

#[test]
fn test_f64_to_int() {
    let v = VectorData::from_vec(vec![1.5_f64, -2.2, 3.9], bitvec![1, 1, 1]).unwrap();
    let vi = v.to_int();
    assert_eq!(vi.data, vec![1, -2, 3]); // truncating cast
}

#[test]
fn test_f64_to_bool() {
    let v = VectorData::from_vec(vec![0.0, 2.5, -0.0], bitvec![1, 1, 1]).unwrap();
    let vb = v.to_bool();
    assert_eq!(vb.data, vec![false, true, false]);
}

#[test]
fn test_f64_to_text() {
    let v = VectorData::from_vec(vec![1.0_f64, -2.5, 0.0], bitvec![1, 1, 1]).unwrap();
    let vs = v.to_text();
    assert_eq!(vs.data, vec!["1".to_string(), "-2.5".to_string(), "0".to_string()]);
}

// ------------------ BOOL CASTS ------------------

#[test]
fn test_bool_to_int_and_float() {
    let v = VectorData::from_vec(vec![true, false, true], bitvec![1, 1, 1]).unwrap();
    let vi = v.to_int();
    let vf = v.to_float();
    assert_eq!(vi.data, vec![1, 0, 1]);
    assert_eq!(vf.data, vec![1.0, 0.0, 1.0]);
}

#[test]
fn test_bool_to_text() {
    let v = VectorData::from_vec(vec![true, false], bitvec![1, 1]).unwrap();
    let vs = v.to_text();
    assert_eq!(vs.data, vec!["true".to_string(), "false".to_string()]);
}

// ------------------ STRING CASTS ------------------

#[test]
fn test_string_to_int_valid_and_invalid() {
    let v = VectorData::from_vec(
        vec!["10".to_string(), "x".to_string(), "30".to_string()],
        bitvec![1, 1, 1],
    )
    .unwrap();
    let vi = v.to_int();
    assert_eq!(vi.data, vec![10, 0, 30]);
    assert_eq!(vi.validity, bitvec![1, 0, 1]); // invalid parse -> false
}

#[test]
fn test_string_to_float_valid_and_invalid() {
    let v = VectorData::from_vec(
        vec!["1.5".to_string(), "foo".to_string(), "-2.0".to_string()],
        bitvec![1, 1, 1],
    )
    .unwrap();
    let vf = v.to_float();
    assert_eq!(vf.data[0], 1.5);
    assert!(vf.data[1].is_nan());
    assert_eq!(vf.data[2], -2.0);
    assert_eq!(vf.validity, bitvec![1, 0, 1]);
}

#[test]
fn test_string_to_bool_true_false_variants() {
    let v = VectorData::from_vec(
        vec!["true".to_string(), "False".to_string(), "yes".to_string()],
        bitvec![1, 1, 1],
    )
    .unwrap();
    let vb = v.to_bool();
    assert_eq!(vb.data, vec![true, false, false]);
    assert_eq!(vb.validity, bitvec![1, 1, 0]);
}