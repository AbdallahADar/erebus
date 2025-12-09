// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

fn v_i64(v: Vec<i64>) -> Vector<i64> {
    Vector::from_vec(v).unwrap()
}

fn v_f64(v: Vec<f64>) -> Vector<f64> {
    Vector::from_vec(v).unwrap()
}

fn v_bool(v: Vec<bool>) -> Vector<bool> {
    Vector::from_vec(v).unwrap()
}

fn v_str(v: Vec<&str>) -> Vector<String> {
    Vector::from_vec(v.into_iter().map(|s| s.to_string()).collect()).unwrap()
}

fn permuted_0_to_n(n: usize) -> Vec<i64> {
    (0..n).map(|i| ((i * 37) % n) as i64).collect()
}

fn sort_algos() -> Vec<Option<&'static str>> {
    vec![
        None,
        Some("introsort"),
        Some("stable"),
        Some("heapsort"),
    ]
}

fn sort_algos_with_radix() -> Vec<Option<&'static str>> {
    vec![
        None,
        Some("introsort"),
        Some("stable"),
        Some("heapsort"),
        Some("radix"),
    ]
}

#[test]
fn test_sort_i64_all_algos() {
    for algo in sort_algos_with_radix() {
        let v = v_i64(vec![5, 1, 9, 3]);

        let s = v.sort(true, algo).unwrap();
        assert_eq!(s.data, vec![1, 3, 5, 9]);

        let s_desc = v.sort(false, algo).unwrap();
        assert_eq!(s_desc.data, vec![9, 5, 3, 1]);
    }
}

#[test]
fn test_sort_f64_all_algos() {
    for algo in sort_algos_with_radix() {
        let v = v_f64(vec![2.2, 9.9, -1.0, 3.3]);

        let s = v.sort(true, algo).unwrap();
        assert_eq!(s.data, vec![-1.0, 2.2, 3.3, 9.9]);

        let s_desc = v.sort(false, algo).unwrap();
        assert_eq!(s_desc.data, vec![9.9, 3.3, 2.2, -1.0]);
    }
}

#[test]
fn test_sort_string_all_algos() {
    for algo in sort_algos() {
        let v = v_str(vec!["pear", "apple", "banana"]);

        let s = v.sort(true, algo).unwrap();
        assert_eq!(s.data, vec!["apple", "banana", "pear"]);

        let s_desc = v.sort(false, algo).unwrap();
        assert_eq!(s_desc.data, vec!["pear", "banana", "apple"]);
    }
}

#[test]
fn test_sort_bool_all_algos() {
    for algo in sort_algos() {
        let v = v_bool(vec![true, false, true, false]);

        let s = v.sort(true, algo).unwrap();
        assert_eq!(s.data, vec![false, false, true, true]);

        let s_desc = v.sort(false, algo).unwrap();
        assert_eq!(s_desc.data, vec![true, true, false, false]);
    }
}

#[test]
fn test_sort_inplace_i64() {
    for algo in sort_algos_with_radix() {
        let mut v = v_i64(vec![5, 1, 9, 3]);

        v.sort_inplace(true, algo).unwrap();
        assert_eq!(v.data, vec![1, 3, 5, 9]);

        v.sort_inplace(false, algo).unwrap();
        assert_eq!(v.data, vec![9, 5, 3, 1]);
    }
}

#[test]
fn test_sort_radix_numeric_only() {
    let v = v_i64(vec![50, -1, 0, 3]);
    let s = v.sort(true, Some("radix")).unwrap();
    assert_eq!(s.data, vec![-1, 0, 3, 50]);

    let v2 = v_f64(vec![2.2, 9.9, -1.0]);
    let s2 = v2.sort(true, Some("radix")).unwrap();
    assert_eq!(s2.data, vec![-1.0, 2.2, 9.9]);
}

#[test]
fn test_sort_radix_non_numeric_fallback() {
    let v = v_str(vec!["c", "a", "b"]);

    // Should NOT panic — should fallback to introsort
    let s = v.sort(true, Some("radix")).unwrap();
    assert_eq!(s.data, vec!["a", "b", "c"]);
}

#[test]
fn test_sort_large_i64_all_algos() {
    let n = 10_000;
    let data = permuted_0_to_n(n);

    for algo in sort_algos_with_radix() {
        let v = v_i64(data.clone());
        let s = v.sort(true, algo).unwrap();

        let expected: Vec<i64> = (0..n as i64).collect();
        assert_eq!(s.data, expected);
    }
}