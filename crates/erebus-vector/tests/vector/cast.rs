// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

#[test]
fn test_is_numeric_i64_and_f64() {
    let vi = Vector::from_vec(vec![1_i64, 2, 3]).unwrap();
    let vf = Vector::from_vec(vec![1.0_f64, 2.0, 3.0]).unwrap();
    let vb = Vector::from_vec(vec![true, false, true]).unwrap();
    let vs = Vector::from_vec(vec!["a".to_string(), "b".to_string(), "c".to_string()]).unwrap();

    assert!(vi.is_numeric());
    assert!(vf.is_numeric());
    assert!(!vb.is_numeric());
    assert!(!vs.is_numeric());
}

// ------------------ i64 CASTS ------------------

#[test]
fn test_i64_to_float() {
    let v = Vector::from_vec(vec![1_i64, 2, 3]).unwrap();
    let vf = v.to_float();
    assert_eq!(vf.data, vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_i64_to_bool() {
    let v = Vector::from_vec(vec![0_i64, 5, -1, 0]).unwrap();
    let vb = v.to_bool();
    assert_eq!(vb.data, vec![false, true, true, false]);
}

#[test]
fn test_i64_to_text() {
    let v = Vector::from_vec(vec![10_i64, -2, 0]).unwrap();
    let vs = v.to_text();
    assert_eq!(vs.data, vec!["10", "-2", "0"]);
}

// ------------------ f64 CASTS ------------------

#[test]
fn test_f64_to_int() {
    let v = Vector::from_vec(vec![1.5_f64, -2.2, 3.9]).unwrap();
    let vi = v.to_int();
    assert_eq!(vi.data, vec![1, -2, 3]); // truncating cast
}

#[test]
fn test_f64_to_bool() {
    let v = Vector::from_vec(vec![0.0, 2.5, -0.0]).unwrap();
    let vb = v.to_bool();
    assert_eq!(vb.data, vec![false, true, false]);
}

#[test]
fn test_f64_to_text() {
    let v = Vector::from_vec(vec![1.0_f64, -2.5, 0.0]).unwrap();
    let vs = v.to_text();
    assert_eq!(vs.data, vec!["1", "-2.5", "0"]);
}

// ------------------ BOOL CASTS ------------------

#[test]
fn test_bool_to_int_and_float() {
    let v = Vector::from_vec(vec![true, false, true]).unwrap();
    let vi = v.to_int();
    let vf = v.to_float();
    assert_eq!(vi.data, vec![1, 0, 1]);
    assert_eq!(vf.data, vec![1.0, 0.0, 1.0]);
}

#[test]
fn test_bool_to_text() {
    let v = Vector::from_vec(vec![true, false]).unwrap();
    let vs = v.to_text();
    assert_eq!(vs.data, vec!["true", "false"]);
}

// ------------------ STRING CASTS ------------------

#[test]
fn test_string_to_int_valid_and_invalid() {
    let v = Vector::from_vec(vec![
        "10".to_string(),
        "x".to_string(),
        "30".to_string(),
    ])
    .unwrap();

    let vi = v.to_int();
    assert_eq!(vi.data, vec![10, i64::MIN, 30]); // invalid parse → sentinel
}

#[test]
fn test_string_to_float_valid_and_invalid() {
    let v = Vector::from_vec(vec![
        "1.5".to_string(),
        "foo".to_string(),
        "-2.0".to_string(),
    ])
    .unwrap();

    let vf = v.to_float();
    assert_eq!(vf.data[0], 1.5);
    assert_eq!(vf.data[1], f64::MIN); // sentinel for invalid
    assert_eq!(vf.data[2], -2.0);
}

#[test]
fn test_string_to_bool_true_false_variants() {
    let v = Vector::from_vec(vec![
        "true".to_string(),
        "False".to_string(),
        "yes".to_string(),
    ])
    .unwrap();

    let vb = v.to_bool();
    assert_eq!(vb.data, vec![true, false, false]);
}


#[test]
fn test_string_to_int_with_invalid_values_uses_sentinel() {
    let v = Vector::from_vec(vec![
        "123".to_string(),
        "NaN".to_string(),
        "hello".to_string(),
    ])
    .unwrap();

    let vi = v.to_int();
    assert_eq!(vi.data, vec![123, i64::MIN, i64::MIN]);
}

#[test]
fn test_string_to_float_with_invalid_values_uses_sentinel() {
    let v = Vector::from_vec(vec![
        "1.5".to_string(),
        "abc".to_string(),
        "NaN".to_string(),
    ])
    .unwrap();

    let vf = v.to_float();
    assert_eq!(vf.data[0], 1.5);
    assert_eq!(vf.data[1], f64::MIN); // invalid parse -> sentinel
    assert_eq!(vf.data[2], f64::MIN); // "NaN" literal -> sentinel
}

#[test]
fn test_bool_to_int_and_float_sentinels_not_used() {
    // Ensure valid casts don't produce sentinel values.
    let v = Vector::from_vec(vec![true, false]).unwrap();
    let vi = v.to_int();
    let vf = v.to_float();
    assert!(vi.data.iter().all(|&x| x != i64::MIN));
    assert!(vf.data.iter().all(|&x| x != f64::MIN));
}

#[test]
fn test_int_and_float_casts_never_produce_sentinels() {
    // When numeric → numeric, sentinel should never appear
    let vi = Vector::from_vec(vec![1_i64, 0, -5]).unwrap();
    let vf = Vector::from_vec(vec![1.0_f64, 0.0, -5.0]).unwrap();

    let vi_to_f = vi.to_float();
    let vf_to_i = vf.to_int();

    assert!(vi_to_f.data.iter().all(|&x| x != f64::MIN));
    assert!(vf_to_i.data.iter().all(|&x| x != i64::MIN));
}

#[test]
fn test_string_to_bool_invalid_uses_false_but_no_sentinel() {
    // Invalid boolean strings are false, not sentinel
    let v = Vector::from_vec(vec![
        "true".to_string(),
        "false".to_string(),
        "maybe".to_string(),
    ])
    .unwrap();

    let vb = v.to_bool();
    assert_eq!(vb.data, vec![true, false, false]);
}