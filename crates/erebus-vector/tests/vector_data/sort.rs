// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

fn vd_i64(v: Vec<i64>, valid: Vec<bool>) -> VectorData<i64> {
    VectorData::from_vec(v, valid.into_iter().collect()).unwrap()
}
fn vd_f64(v: Vec<f64>, valid: Vec<bool>) -> VectorData<f64> {
    VectorData::from_vec(v, valid.into_iter().collect()).unwrap()
}

fn vd_bool(data: Vec<bool>, valid: Vec<bool>) -> VectorData<bool> {
    VectorData::from_vec(data, BitVec::from_iter(valid)).unwrap()
}

fn vd_str(v: Vec<&str>, valid: Vec<bool>) -> VectorData<String> {
    VectorData::from_vec(v.into_iter().map(|s| s.to_string()).collect(),
    valid.into_iter().collect()).unwrap()
}

fn permuted_0_to_n(n: usize) -> Vec<i64> {
    // 37 is coprime to many reasonable n (including 10_000), so this is a permutation.
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
        let vd = vd_i64(vec![5, 1, 9, 3], vec![true, true, true, true]);

        let sorted = vd.sort(true, true, algo).unwrap();
        assert_eq!(sorted.data, vec![1, 3, 5, 9]);
        assert_eq!(sorted.validity.to_vec(), bitvec![1, 1, 1, 1]);

        let sorted_desc = vd.sort(false, true, algo).unwrap();
        assert_eq!(sorted_desc.data, vec![9, 5, 3, 1]);
    }
}

#[test]
fn test_sort_i64_with_nulls_all_algos() {
    for algo in sort_algos_with_radix() {
        let vd = vd_i64(vec![5, 1, 9, 3], vec![false, true, true, false]);

        let sorted = vd.sort(true, true, algo).unwrap();

        assert_eq!(sorted.data, vec![1, 9, 5, 3]);
        assert_eq!(sorted.validity.to_vec(), bitvec![1, 1, 0, 0]);
    }
}

#[test]
fn test_sort_f64_all_algos() {
    for algo in sort_algos_with_radix() {
        let vd = vd_f64(vec![2.2, 9.9, -1.0, 3.3], vec![true; 4]);

        let sorted = vd.sort(true, true, algo).unwrap();
        assert_eq!(sorted.data, vec![-1.0, 2.2, 3.3, 9.9]);

        let sorted_desc = vd.sort(false, true, algo).unwrap();
        assert_eq!(sorted_desc.data, vec![9.9, 3.3, 2.2, -1.0]);
    }
}

#[test]
fn test_sort_f64_with_nulls_all_algos() {
    for algo in sort_algos_with_radix() {
        let vd = vd_f64(vec![3.3, 1.1, 9.9, 0.0], vec![true, false, true, false]);

        let sorted = vd.sort(true, true, algo).unwrap();

        assert_eq!(sorted.data, vec![3.3, 9.9, 1.1, 0.0]);
        assert_eq!(sorted.validity.to_vec(), bitvec![1, 1, 0, 0]);
    }
}

#[test]
fn test_sort_string_all_algos_except_radix() {
    for algo in sort_algos() {
        let vd = vd_str(vec!["pear", "apple", "banana"], vec![true; 3]);

        let sorted = vd.sort(true, true, algo).unwrap();
        assert_eq!(sorted.data, vec!["apple", "banana", "pear"]);

        let sorted_desc = vd.sort(false, true, algo).unwrap();
        assert_eq!(sorted_desc.data, vec!["pear", "banana", "apple"]);
    }
}

#[test]
fn test_sort_bool_all_algos_except_radix() {
    for algo in sort_algos() {
        let vd = vd_bool(vec![true, false, true, false], vec![true; 4]);

        let sorted = vd.sort(true, true, algo).unwrap();
        assert_eq!(sorted.data, vec![false, false, true, true]);

        let sorted_desc = vd.sort(false, true, algo).unwrap();
        assert_eq!(sorted_desc.data, vec![true, true, false, false]);
    }
}

#[test]
fn test_sort_inplace_i64() {
    for algo in sort_algos_with_radix() {
        let mut vd = vd_i64(vec![5, 1, 9, 3], vec![true; 4]);

        vd.sort_inplace(true, true, algo).unwrap();
        assert_eq!(vd.data, vec![1, 3, 5, 9]);

        vd.sort_inplace(false, true, algo).unwrap();
        assert_eq!(vd.data, vec![9, 5, 3, 1]);
    }
}

#[test]
fn test_sort_string_with_nulls_all_algos() {
    for algo in sort_algos() {
        let vd = vd_str(vec!["b", "a", "c"], vec![true, false, true]);

        let sorted = vd.sort(true, true, algo).unwrap();

        assert_eq!(sorted.data, vec!["b", "c", "a"]);
        assert_eq!(sorted.validity.to_vec(), bitvec![1, 1, 0]);
    }
}

#[test]
fn test_sort_i64_nulls_first_all_algos() {
    for algo in sort_algos_with_radix() {
        let vd = vd_i64(vec![5, 1, 9, 3], vec![false, true, true, false]);

        let sorted = vd.sort(true, false, algo).unwrap();

        // nulls_first: nulls (5,3) first in original order, then valid (1,9) sorted
        assert_eq!(sorted.data, vec![5, 3, 1, 9]);
        assert_eq!(sorted.validity.to_vec(), bitvec![0, 0, 1, 1]);
    }
}

#[test]
fn test_sort_string_with_nulls_first_all_algos() {
    for algo in sort_algos() {
        let vd = vd_str(vec!["b", "a", "c"], vec![true, false, true]);

        let sorted = vd.sort(true, false, algo).unwrap();

        assert_eq!(sorted.data, vec!["a", "b", "c"]);
        assert_eq!(sorted.validity.to_vec(), bitvec![0, 1, 1]);
    }
}

#[test]
fn test_sort_bool_with_nulls_all_algos() {
    for algo in sort_algos() {
        let vd = vd_bool(vec![true, false, true], vec![false, true, true]);

        let sorted = vd.sort(true, true, algo).unwrap();

        assert_eq!(sorted.data, vec![false, true, true]);
        assert_eq!(sorted.validity.to_vec(), bitvec![1, 1, 0]);
    }
}

#[test]
fn test_sort_inplace_f64_with_nulls_all_algos() {
    for algo in sort_algos_with_radix() {
        let mut vd = vd_f64(
            vec![3.3, 1.1, 9.9, 0.0],
            vec![true, false, true, false]
        );

        vd.sort_inplace(true, true, algo).unwrap();

        assert_eq!(vd.data, vec![3.3, 9.9, 1.1, 0.0]);
        assert_eq!(vd.validity.to_vec(), bitvec![1, 1, 0, 0]);
    }
}

#[test]
fn test_sort_radix_numeric_only() {
    let vd = vd_i64(vec![50, -1, 0, 3], vec![true; 4]);
    let sorted = vd.sort(true, true, Some("radix")).unwrap();
    assert_eq!(sorted.data, vec![-1, 0, 3, 50]);

    let vd2 = vd_f64(vec![2.2, 9.9, -1.0], vec![true; 3]);
    let sorted2 = vd2.sort(true, true, Some("radix")).unwrap();
    assert_eq!(sorted2.data, vec![-1.0, 2.2, 9.9]);
}

#[test]
fn test_sort_radix_non_numeric_fallback() {
    let vd = vd_str(vec!["c", "a", "b"], vec![true; 3]);

    // Should NOT panic — should fallback to introsort
    let sorted = vd.sort(true, true, Some("radix")).unwrap();
    assert_eq!(sorted.data, vec!["a", "b", "c"]);
}

#[test]
fn test_sort_large_i64_all_algos() {
    let n = 10_000;
    let data = permuted_0_to_n(n);
    let validity = vec![true; n];

    for algo in sort_algos_with_radix() {
        let vd = vd_i64(data.clone(), validity.clone());

        let sorted = vd.sort(true, true, algo).unwrap();

        // Expect 0..n-1 in order
        let expected: Vec<i64> = (0..n as i64).collect();
        assert_eq!(sorted.data, expected);
        assert_eq!(sorted.validity.to_vec(), bitvec![1; n]);
    }
}

#[test]
fn test_sort_large_i64_with_nulls_all_algos() {
    let n = 1_000;
    let data = permuted_0_to_n(n);
    // Mark every 3rd element as null.
    let validity: Vec<bool> = (0..n).map(|i| i % 3 != 0).collect();

    for algo in sort_algos_with_radix() {
        let vd = vd_i64(data.clone(), validity.clone());

        let sorted = vd.sort(true, true, algo).unwrap(); // nulls_last = true

        // All valid values should be sorted ascending, nulls at the end.
        let mut valid_vals: Vec<i64> = data
            .iter()
            .zip(validity.iter())
            .filter_map(|(v, &is_valid)| if is_valid { Some(*v) } else { None })
            .collect();
        valid_vals.sort();

        let valid_count = valid_vals.len();

        assert_eq!(&sorted.data[..valid_count], &valid_vals[..]);
        assert!(sorted.validity[..valid_count].iter().all(|b| *b));
        assert!(sorted.validity[valid_count..].iter().all(|b| !*b));
    }
}