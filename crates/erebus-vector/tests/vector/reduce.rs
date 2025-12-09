// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

pub fn v_i64(v: Vec<i64>) -> Vector<i64> {
    Vector { data: v }
}
pub fn v_f64(v: Vec<f64>) -> Vector<f64> {
    Vector { data: v }
}

fn v_bool(v: Vec<bool>) -> Vector<bool> { Vector { data: v } }


#[test]
fn test_sum_vector() {
    let v = v_i64(vec![1,2,3,4]);
    assert_eq!(v.sum(), 10);

    let f = v_f64(vec![1.0,2.5,3.5]);
    assert!((f.sum() - 7.0).abs() < 1e-9);
}

#[test]
fn test_prod_vector() {
    let v = v_i64(vec![2,3,4]);
    assert_eq!(v.prod(), 24);

    let f = v_f64(vec![1.5, 2.0]);
    assert!((f.prod() - 3.0).abs() < 1e-9);
}

#[test]
fn test_sumsq_vector() {
    let v = v_i64(vec![1,2,3]);
    assert_eq!(v.sum_of_squares(), 14.0);

    let f = v_f64(vec![1.0, -2.0]);
    assert_eq!(f.sum_of_squares(), 5.0);
}

#[test]
fn test_mean_vector() {
    let v = v_i64(vec![1,2,3]);
    assert_eq!(v.mean(), 2.0);
}

#[test]
fn test_l0_norm_vector() {
    let v = v_i64(vec![1,0,5]);
    assert_eq!(v.l0_norm(), 2);
}

#[test]
fn test_l1_norm_vector() {
    let v = v_i64(vec![1,-2,3]);
    assert_eq!(v.l1_norm(), 6);
}

#[test]
fn test_l2_norm_vector() {
    let v = v_f64(vec![3.0,4.0]);
    assert_eq!(v.l2_norm(), 5.0);
}

#[test]
fn test_lp_norm_vector() {
    let v = v_f64(vec![1.0,2.0,3.0]);
    let lp = v.lp_norm(3.0);
    assert!((lp - (1.0_f64 + 8.0_f64 + 27.0_f64).powf(1.0_f64/3.0_f64)).abs() < 1e-9);
}

#[test]
fn test_linf_norm_vector() {
    let v = v_f64(vec![1.0,-5.0,3.0]);
    assert_eq!(v.linf_norm(), 5.0);
}

#[test]
fn test_geometric_mean_vector() {
    let v = v_f64(vec![1.0,4.0,9.0]);
    assert!((v.geometric_mean() - 3.3019).abs() < 1e-3);
}

#[test]
fn test_harmonic_mean_vector() {
    let v = v_f64(vec![1.0,2.0,4.0]);
    assert!((v.harmonic_mean() - (3.0/1.75)).abs() < 1e-9);
}

#[test]
fn test_skewness_vector() {
    let v = v_f64(vec![1.0,2.0,3.0,4.0,5.0]);
    assert!((v.skewness() - 0.0).abs() < 1e-9);
}

#[test]
fn test_kurtosis_vector() {
    let v = v_f64(vec![1.0,2.0,3.0,4.0,5.0]);
    assert!(v.kurtosis() < 0.0);
}

#[test]
fn test_var_vector() {
    let v = v_f64(vec![1.0,2.0,3.0]);
    assert_eq!(v.var(Some(1)), 1.0);
}

#[test]
fn test_std_vector() {
    let v = v_f64(vec![1.0,2.0,3.0]);
    assert_eq!(v.std(Some(1)), 1.0);
}

#[test]
fn test_vector_any_all() {
    let v = v_bool(vec![true, false, true]);
    assert!(v.any());
    assert!(!v.all());

    let v2 = v_bool(vec![true, true]);
    assert!(v2.any());
    assert!(v2.all());

    let v3 = v_bool(vec![false, false]);
    assert!(!v3.any());
    assert!(!v3.all());
}

#[test]
fn test_count_true_false_vector() {
    let v = v_bool(vec![true, false, true]);
    assert_eq!(v.count_true(), 2);
    assert_eq!(v.count_false(), 1);
}

#[test]
fn test_null_ratio_vector_basic() {
    let v = v_i64(vec![10, 20, 30]);

    assert_eq!(v.null_ratio(), 0.0);
    assert_eq!(v.non_null_ratio(), 1.0);
    assert_eq!(v.null_percentage(), 0.0);
    assert_eq!(v.non_null_percentage(), 100.0);
}

#[test]
fn test_null_ratio_vector_empty() {
    let v = v_i64(vec![]);

    assert_eq!(v.null_ratio(), 0.0);
    assert_eq!(v.non_null_ratio(), 1.0);
    assert_eq!(v.null_percentage(), 0.0);
    assert_eq!(v.non_null_percentage(), 100.0);
}

#[test]
fn test_unique_vector_basic() {
    let v = v_i64(vec![10, 20, 10, 30, 20]);
    let u = v.unique();
    assert_eq!(u.data, vec![10, 20, 30]);
}

#[test]
fn test_unique_vector_order() {
    let v = v_i64(vec![5, 3, 5, 2, 3, 9]);
    let u = v.unique();
    assert_eq!(u.data, vec![5, 3, 2, 9]);
}

#[test]
fn test_unique_vector_empty() {
    let v = Vector::<i64>::empty();
    let u = v.unique();
    assert!(u.data.is_empty());
}

#[test]
fn test_n_unique_vector() {
    let v = v_i64(vec![10, 20, 10, 20, 30]);
    assert_eq!(v.n_unique(), 3);
    assert_eq!(v.n_distinct(), 3);
}

#[test]
fn test_vector_f64_unique_basic() {
    let v = v_f64(vec![1.0, 2.0, 1.0, 3.0, 2.0]);

    let u = v.unique();

    assert_eq!(u.data, vec![1.0, 2.0, 3.0]);
    assert_eq!(v.n_unique(), 3);
}

#[test]
fn test_vector_f64_unique_negatives() {
    let v = v_f64(vec![-1.0, -1.0, -2.0, -3.0, -2.0]);
    let u = v.unique();

    assert_eq!(u.data, vec![-1.0, -2.0, -3.0]);
    assert_eq!(v.n_unique(), 3);
}

#[test]
fn test_vector_f64_unique_with_nan() {
    let nan = f64::NAN;
    let v = v_f64(vec![1.0, nan, nan, 2.0, nan]);

    let u = v.unique();

    // OrderedFloat makes all NaNs equal, so only ONE NaN appears
    assert_eq!(u.data.len(), 3);

    // Verify presence of NaN using is_nan()
    assert!(u.data[1].is_nan());

    assert!(u.data.contains(&1.0));
    assert!(u.data.contains(&2.0));

    assert_eq!(v.n_unique(), 3);
}

#[test]
fn test_vector_f64_unique_empty() {
    let v = v_f64(vec![]);
    let u = v.unique();

    assert!(u.data.is_empty());
    assert_eq!(v.n_unique(), 0);
}

#[test]
fn test_vec_max_basic_i64() {
    let v = v_i64(vec![3, 1, 10, 7]);

    assert_eq!(v.max(), Some(10));
    assert_eq!(v.argmax(), Some(2));
}

#[test]
fn test_vec_max_single() {
    let v = v_i64(vec![42]);

    assert_eq!(v.max(), Some(42));
    assert_eq!(v.argmax(), Some(0));
}

#[test]
fn test_vec_max_empty() {
    let v = v_i64(Vec::<i64>::new());

    assert_eq!(v.max(), None);
    assert_eq!(v.argmax(), None);
}

#[test]
fn test_vec_max_basic_f64() {
    let v = v_f64(vec![1.2, 9.9, 5.1]);

    assert_eq!(v.max(), Some(9.9));
    assert_eq!(v.argmax(), Some(1));
}

#[test]
fn test_vec_max_negative_f64() {
    let v = v_f64(vec![-10.0, -3.0, -7.0]);

    assert_eq!(v.max(), Some(-3.0));
    assert_eq!(v.argmax(), Some(1));
}

#[test]
fn test_vec_max_with_index_basic() {
    let v = v_i64(vec![4, 9, 1]);

    let (val, idx) = v.max_with_index();
    assert_eq!(val.unwrap(), 9);
    assert_eq!(idx.unwrap(), 1);
}

#[test]
fn test_vec_max_with_index_f64() {
    let v = v_f64(vec![1.5, 3.2, 2.9]);

    let (val, idx) = v.max_with_index();
    assert_eq!(val.unwrap(), 3.2);
    assert_eq!(idx.unwrap(), 1);
}

#[test]
fn test_vec_max_with_index_empty() {
    let v = v_i64(Vec::<i64>::new());
    assert_eq!(v.max_with_index(), (None, None));
}

#[test]
fn test_vec_min_basic_i64() {
    let v = v_i64(vec![3, 1, 10, 7]);

    assert_eq!(v.min(), Some(1));
    assert_eq!(v.argmin(), Some(1));
}

#[test]
fn test_vec_min_single() {
    let v = v_i64(vec![42]);

    assert_eq!(v.min(), Some(42));
    assert_eq!(v.argmin(), Some(0));
}

#[test]
fn test_vec_min_empty() {
    let v = v_i64(Vec::<i64>::new());

    assert_eq!(v.min(), None);
    assert_eq!(v.argmin(), None);
}

#[test]
fn test_vec_min_basic_f64() {
    let v = v_f64(vec![1.2, 9.9, 5.1]);

    assert_eq!(v.min(), Some(1.2));
    assert_eq!(v.argmin(), Some(0));
}

#[test]
fn test_vec_min_negative_f64() {
    let v = v_f64(vec![-10.0, -3.0, -7.0]);

    assert_eq!(v.min(), Some(-10.0));
    assert_eq!(v.argmin(), Some(0));
}

#[test]
fn test_vec_min_with_index_basic() {
    let v = v_i64(vec![4, 9, 1]);

    let (val, idx) = v.min_with_index();
    assert_eq!(val.unwrap(), 1);
    assert_eq!(idx.unwrap(), 2);
}

#[test]
fn test_vec_min_with_index_f64() {
    let v = v_f64(vec![1.5, 3.2, 0.9]);

    let (val, idx) = v.min_with_index();
    assert_eq!(val.unwrap(), 0.9);
    assert_eq!(idx.unwrap(), 2);
}

#[test]
fn test_vec_min_with_index_empty() {
    let v = v_i64(Vec::<i64>::new());
    assert_eq!(v.min_with_index(), (None, None));
}

#[test]
fn test_vec_min_with_index_all_equal() {
    let v = v_i64(vec![5, 5, 5, 5]);

    let (val, idx) = v.min_with_index();
    assert_eq!(val.unwrap(), 5);
    assert_eq!(idx.unwrap(), 0); // first occurrence
}