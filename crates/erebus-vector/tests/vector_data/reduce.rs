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

#[test]
fn test_sum_i64_and_f64() {
    let v = vd_i64(vec![1,2,3,4], vec![true;4]);
    assert_eq!(v.sum(), 10);

    let f = vd_f64(vec![1.0,2.5,3.5], vec![true;3]);
    assert!((f.sum() - 7.0).abs() < 1e-9);
}

#[test]
fn test_sum_with_nulls() {
    let v = vd_i64(vec![1,2,3], vec![true,false,true]);
    assert_eq!(v.sum(), 4);  // skip middle
}

#[test]
fn test_prod_i64_and_f64() {
    let v = vd_i64(vec![2,3,4], vec![true;3]);
    assert_eq!(v.prod(), 24);

    let f = vd_f64(vec![1.5, 2.0], vec![true;2]);
    assert!((f.prod() - 3.0).abs() < 1e-9);
}

#[test]
fn test_prod_with_nulls() {
    let v = vd_i64(vec![2, 3, 5], vec![true, false, true]);
    assert_eq!(v.prod(), 10);
}

#[test]
fn test_sumsq_i64_and_f64() {
    let v = vd_i64(vec![1,2,3], vec![true;3]);
    assert_eq!(v.sum_of_squares(), 14.0);

    let f = vd_f64(vec![1.0, -2.0], vec![true;2]);
    assert_eq!(f.sum_of_squares(), 5.0);
}

#[test]
fn test_mean_basic() {
    let v = vd_i64(vec![1,2,3], vec![true;3]);
    assert_eq!(v.mean(), 2.0);

    let f = vd_f64(vec![2.0,4.0,6.0], vec![true;3]);
    assert_eq!(f.mean(), 4.0);
}

#[test]
fn test_mean_with_nulls() {
    let v = vd_i64(vec![1,100,3], vec![true,false,true]);
    assert_eq!(v.mean(), 2.0);
}

#[test]
fn test_mean_all_null() {
    let v = vd_i64(vec![1,2,3], vec![false,false,false]);
    assert!(v.mean().is_nan());
}

#[test]
fn test_l0_norm() {
    let v = vd_i64(vec![1,0,5], vec![true;3]);
    assert_eq!(v.l0_norm(), 2);

    let vn = vd_i64(vec![1,0,5], vec![true,false,true]);
    assert_eq!(vn.l0_norm(), 2);
}

#[test]
fn test_l1_norm() {
    let v = vd_i64(vec![1,-2,3], vec![true;3]);
    assert_eq!(v.l1_norm(), 6);
}

#[test]
fn test_l2_norm() {
    let v = vd_f64(vec![3.0, 4.0], vec![true;2]);
    assert_eq!(v.l2_norm(), 5.0);
}

#[test]
fn test_lp_norm() {
    let v = vd_f64(vec![1.0, 2.0, 3.0], vec![true;3]);
    let lp = v.lp_norm(3.0);
    assert!((lp - (1f64.powi(3)+8.0+27.0).powf(1.0/3.0)).abs() < 1e-9);
}

#[test]
fn test_linf_norm() {
    let v = vd_f64(vec![1.0, -5.0, 3.0], vec![true;3]);
    assert_eq!(v.linf_norm(), 5.0);
}

#[test]
fn test_geometric_mean() {
    let v = vd_f64(vec![1.0,4.0,9.0], vec![true;3]);
    assert!((v.geometric_mean() - 3.3019).abs() < 1e-4);
}

#[test]
fn test_geometric_mean_nulls() {
    let v = vd_f64(vec![1.0,4.0,9.0], vec![false,true,true]);
    assert!((v.geometric_mean() - 6.0).abs() < 1e-9);
}

#[test]
fn test_geometric_mean_invalid() {
    let v = vd_f64(vec![1.0,-2.0,4.0], vec![true;3]);
    assert!(v.geometric_mean().is_nan());
}

#[test]
fn test_harmonic_mean() {
    let v = vd_f64(vec![1.0, 2.0, 4.0], vec![true;3]);
    // HM = 3 / (1 + .5 + .25) = 3/1.75
    assert!((v.harmonic_mean() - (3.0/1.75)).abs() < 1e-9);
}

#[test]
fn test_skewness() {
    let v = vd_f64(vec![1.0,2.0,3.0,4.0,5.0], vec![true;5]);
    let sk = v.skewness();
    assert!((sk - 0.0).abs() < 1e-9);
}

#[test]
fn test_kurtosis() {
    let v = vd_f64(vec![1.0,2.0,3.0,4.0,5.0], vec![true;5]);
    let k = v.kurtosis();
    // Uniform-ish distribution → negative kurtosis
    assert!(k < 0.0);
}

#[test]
fn test_var_population() {
    let v = vd_f64(vec![1.0,2.0,3.0], vec![true;3]);
    assert_eq!(v.var(Some(0)), 2.0/3.0);
}

#[test]
fn test_var_sample() {
    let v = vd_f64(vec![1.0,2.0,3.0], vec![true;3]);
    assert_eq!(v.var(Some(1)), 1.0);
}

#[test]
fn test_std() {
    let v = vd_f64(vec![1.0,2.0,3.0], vec![true;3]);
    assert_eq!(v.std(Some(1)), 1.0);
}

#[test]
fn test_vector_data_any_all() {
    let vd = vd_bool(vec![true, false, true], vec![true, true, true]);
    assert!(vd.any());
    assert!(!vd.all());

    let vd2 = vd_bool(vec![true, true, true], vec![true, true, true]);
    assert!(vd2.any());
    assert!(vd2.all());

    let vd3 = vd_bool(vec![true, false, true], vec![false, false, false]);
    assert!(!vd3.any());  // all null → no valid true
    assert!(vd3.all());   // all null → vacuously true
}

#[test]
fn test_count_true_false_vectordata() {
    let vd = vd_bool(
        vec![true, false, true, false],
        vec![true, true, false, true]  // third value is null
    );

    assert_eq!(vd.count_true(), 1);   // only index 0
    assert_eq!(vd.count_false(), 2);  // index 1 and 3
}

#[test]
fn test_null_ratio_vectordata_basic() {
    let vd = vd_i64(vec![1,2,3,4], vec![true, false, true, false]);

    assert!((vd.null_ratio() - 0.5).abs() < 1e-9);
    assert!((vd.non_null_ratio() - 0.5).abs() < 1e-9);
}

#[test]
fn test_null_ratio_vectordata_empty() {
    let vd = vd_i64(vec![], vec![]);
    assert_eq!(vd.null_ratio(), 0.0);
    assert_eq!(vd.non_null_ratio(), 1.0);
    assert_eq!(vd.null_percentage(), 0.0);
    assert_eq!(vd.non_null_percentage(), 100.0);
}

#[test]
fn test_null_percentage_vectordata() {
    let vd = vd_i64(vec![1,2,3], vec![false, true, false]);

    assert!((vd.null_ratio() - 2.0/3.0).abs() < 1e-9);
    assert!((vd.null_percentage() - (2.0/3.0 * 100.0)).abs() < 1e-9);

    assert!((vd.non_null_ratio() - (1.0/3.0)).abs() < 1e-9);
    assert!((vd.non_null_percentage() - (1.0/3.0 * 100.0)).abs() < 1e-9);
}

#[test]
fn test_unique_vectordata_basic() {
    let vd = vd_i64(vec![10, 20, 10, 30, 20], vec![true; 5]);

    let u = vd.unique();
    assert_eq!(u.data, vec![10, 20, 30]);
    assert_eq!(u.validity.to_vec(), bitvec![1,1,1]);
}

#[test]
fn test_unique_vectordata_preserves_order() {
    let vd = vd_i64(vec![5, 3, 5, 2, 3, 9], vec![true; 6]);

    let u = vd.unique();
    assert_eq!(u.data, vec![5, 3, 2, 9]);
}

#[test]
fn test_unique_vectordata_skips_nulls() {
    let vd = vd_i64(vec![1, 2, 3, 2, 1], vec![true, false, true, true, false]);

    // valid slice = [1, -, 3, 2, -]
    // uniques of valid = [1, 3, 2]
    let u = vd.unique();
    assert_eq!(u.data, vec![1, 3, 2]);
    assert_eq!(u.validity.to_vec(), bitvec![1,1,1]);
}

#[test]
fn test_unique_vectordata_empty() {
    let vd = VectorData::<i64>::empty();
    let u = vd.unique();
    assert!(u.data.is_empty());
    assert!(u.validity.is_empty());
}

#[test]
fn test_n_unique_vectordata() {
    let vd = vd_i64(vec![10, 20, 10, 20, 30], vec![true; 5]);
    assert_eq!(vd.n_unique(), 3);
    assert_eq!(vd.n_distinct(), 3);
}

#[test]
fn test_n_unique_vectordata_nulls() {
    let vd = vd_i64(vec![1, 2, 1, 2, 3], vec![true, false, true, true, false]);
    // valid = [1, -, 1, 2, -] → uniques = {1, 2}
    assert_eq!(vd.n_unique(), 2);
}

#[test]
fn test_vd_f64_unique_basic() {
    let vd = vd_f64(
        vec![1.0, 2.0, 1.0, 3.0, 2.0],
        vec![true, true, true, true, true]
    );

    let u = vd.unique();

    assert_eq!(u.data, vec![1.0, 2.0, 3.0]);
    assert_eq!(vd.n_unique(), 3);
}

#[test]
fn test_vd_f64_unique_nulls() {
    // Only two valid entries: 1.0 and 2.0 → unique = 2
    let vd = vd_f64(
        vec![1.0, 99.0, 2.0, 99.0],
        vec![true, false, true, false]
    );

    let u = vd.unique();

    assert_eq!(u.data, vec![1.0, 2.0]);
    assert_eq!(u.validity.to_vec(), bitvec![1, 1]);
    assert_eq!(vd.n_unique(), 2);
}

#[test]
fn test_vd_f64_unique_with_nan() {
    let nan = f64::NAN;

    // Valid entries: 1.0, NaN, 2.0, NaN → unique = {1.0, NaN, 2.0}
    let vd = vd_f64(
        vec![1.0, nan, 2.0, nan],
        vec![true, true, true, true]
    );

    let u = vd.unique();

    assert_eq!(u.data.len(), 3);
    assert!(u.data[1].is_nan());
    assert!(u.data.contains(&1.0));
    assert!(u.data.contains(&2.0));

    assert_eq!(vd.n_unique(), 3);
}

#[test]
fn test_vd_f64_unique_all_null() {
    let vd = vd_f64(
        vec![1.0, 2.0, 3.0],
        vec![false, false, false]
    );

    let u = vd.unique();

    assert_eq!(u.data.len(), 0);
    assert_eq!(u.validity.len(), 0);
    assert_eq!(vd.n_unique(), 0);
}

#[test]
fn test_vd_f64_unique_empty() {
    let vd = vd_f64(vec![], vec![]);

    let u = vd.unique();

    assert!(u.data.is_empty());
    assert!(u.validity.is_empty());
    assert_eq!(vd.n_unique(), 0);
}

#[test]
fn test_vd_max_basic_i64() {
    let vd = vd_i64(vec![3, 1, 10, 7], vec![true, true, true, true]);

    assert_eq!(vd.max(), Some(10));
    assert_eq!(vd.argmax(), Some(2));
}

#[test]
fn test_vd_max_with_nulls_i64() {
    let vd = vd_i64(vec![3, 1, 10, 7], vec![false, true, false, true]);

    assert_eq!(vd.max(), Some(7));
    assert_eq!(vd.argmax(), Some(3));
}

#[test]
fn test_vd_max_all_null_i64() {
    let vd = vd_i64(vec![3, 1, 10], vec![false, false, false]);

    assert_eq!(vd.max(), None);
    assert_eq!(vd.argmax(), None);
}

#[test]
fn test_vd_max_empty_i64() {
    let vd = vd_i64(vec![], vec![]);

    assert_eq!(vd.max(), None);
    assert_eq!(vd.argmax(), None);
}

#[test]
fn test_vd_max_f64_basic() {
    let vd = vd_f64(vec![1.5, 9.3, 2.2], vec![true, true, true]);

    assert_eq!(vd.max(), Some(9.3));
    assert_eq!(vd.argmax(), Some(1));
}

#[test]
fn test_vd_max_with_nulls_f64() {
    let vd = vd_f64(vec![5.1, 2.2, 12.8, 7.4], vec![true, false, true, false]);

    assert_eq!(vd.max(), Some(12.8));
    assert_eq!(vd.argmax(), Some(2));
}

#[test]
fn test_vd_max_f64_all_nulls() {
    let vd = vd_f64(vec![1.1, 3.3], vec![false, false]);

    assert_eq!(vd.max(), None);
    assert_eq!(vd.argmax(), None);
}

#[test]
fn test_vd_max_with_index_basic_i64() {
    let vd = vd_i64(vec![3, 1, 10, 7], vec![true, true, true, true]);

    let out = vd.max_with_index();

    let (val, idx) = out;
    assert_eq!(val.unwrap(), 10);
    assert_eq!(idx.unwrap(), 2);
}

#[test]
fn test_vd_max_with_index_with_nulls_i64() {
    let vd = vd_i64(vec![3, 1, 10, 7], vec![false, true, false, true]);

    let out = vd.max_with_index();
    assert_eq!(out, (Some(7), Some(3)));
}

#[test]
fn test_vd_max_with_index_all_null_i64() {
    let vd = vd_i64(vec![3, 1, 10], vec![false, false, false]);

    assert_eq!(vd.max_with_index(), (None, None));
}

#[test]
fn test_vd_max_with_index_empty_i64() {
    let vd = vd_i64(vec![], vec![]);

    assert_eq!(vd.max_with_index(), (None, None));
}

#[test]
fn test_vd_max_with_index_basic_f64() {
    let vd = vd_f64(vec![1.5, 9.3, 2.2], vec![true, true, true]);

    let (val, idx) = vd.max_with_index();
    assert_eq!(val.unwrap(), 9.3);
    assert_eq!(idx.unwrap(), 1);
}

#[test]
fn test_vd_max_with_index_with_nulls_f64() {
    let vd = vd_f64(vec![5.1, 2.2, 12.8, 7.4], vec![true, false, true, false]);

    let (val, idx) = vd.max_with_index();
    assert_eq!(val.unwrap(), 12.8);
    assert_eq!(idx.unwrap(), 2);
}

#[test]
fn test_vd_max_with_index_all_null_f64() {
    let vd = vd_f64(vec![1.1, 3.3], vec![false, false]);

    assert_eq!(vd.max_with_index(), (None, None));
}

#[test]
fn test_vd_max_with_index_sparse_valid() {
    let vd = vd_i64(
        vec![100, 5, 999, 1, 888, 777],
        vec![false, true, false, false, true, false],
    );

    let (val, idx) = vd.max_with_index();
    assert_eq!(val.unwrap(), 888);
    assert_eq!(idx.unwrap(), 4);
}

#[test]
fn test_vd_min_basic_i64() {
    let vd = vd_i64(vec![3, 1, 10, 7], vec![true, true, true, true]);

    assert_eq!(vd.min(), Some(1));
    assert_eq!(vd.argmin(), Some(1));
}

#[test]
fn test_vd_min_with_nulls_i64() {
    let vd = vd_i64(vec![3, 1, 10, 7], vec![false, true, false, true]);

    assert_eq!(vd.min(), Some(1));
    assert_eq!(vd.argmin(), Some(1));
}

#[test]
fn test_vd_min_all_null_i64() {
    let vd = vd_i64(vec![3, 1, 10], vec![false, false, false]);

    assert_eq!(vd.min(), None);
    assert_eq!(vd.argmin(), None);
}

#[test]
fn test_vd_min_empty_i64() {
    let vd = vd_i64(vec![], vec![]);

    assert_eq!(vd.min(), None);
    assert_eq!(vd.argmin(), None);
}

#[test]
fn test_vd_min_f64_basic() {
    let vd = vd_f64(vec![1.5, 9.3, 2.2], vec![true, true, true]);

    assert_eq!(vd.min(), Some(1.5));
    assert_eq!(vd.argmin(), Some(0));
}

#[test]
fn test_vd_min_with_nulls_f64() {
    let vd = vd_f64(vec![5.1, 2.2, 12.8, 7.4], vec![true, false, true, false]);

    assert_eq!(vd.min(), Some(5.1));
    assert_eq!(vd.argmin(), Some(0));
}

#[test]
fn test_vd_min_f64_all_nulls() {
    let vd = vd_f64(vec![1.1, 3.3], vec![false, false]);

    assert_eq!(vd.min(), None);
    assert_eq!(vd.argmin(), None);
}

#[test]
fn test_vd_min_with_index_basic_i64() {
    let vd = vd_i64(vec![3, 1, 10, 7], vec![true, true, true, true]);

    let (val, idx) = vd.min_with_index();
    assert_eq!(val.unwrap(), 1);
    assert_eq!(idx.unwrap(), 1);
}

#[test]
fn test_vd_min_with_index_with_nulls_i64() {
    let vd = vd_i64(vec![3, 1, 10, 7], vec![false, true, false, true]);

    let (val, idx) = vd.min_with_index();
    assert_eq!(val.unwrap(), 1);
    assert_eq!(idx.unwrap(), 1);
}

#[test]
fn test_vd_min_with_index_all_null_i64() {
    let vd = vd_i64(vec![3, 1, 10], vec![false, false, false]);

    assert_eq!(vd.min_with_index(), (None, None));
}

#[test]
fn test_vd_min_with_index_empty_i64() {
    let vd = vd_i64(vec![], vec![]);

    assert_eq!(vd.min_with_index(), (None, None));
}

#[test]
fn test_vd_min_with_index_basic_f64() {
    let vd = vd_f64(vec![1.5, 9.3, 2.2], vec![true, true, true]);

    let (val, idx) = vd.min_with_index();
    assert_eq!(val.unwrap(), 1.5);
    assert_eq!(idx.unwrap(), 0);
}

#[test]
fn test_vd_min_with_index_with_nulls_f64() {
    let vd = vd_f64(vec![5.1, 2.2, 12.8, 7.4], vec![true, false, true, false]);

    let (val, idx) = vd.min_with_index();
    assert_eq!(val.unwrap(), 5.1);
    assert_eq!(idx.unwrap(), 0);
}

#[test]
fn test_vd_min_with_index_all_null_f64() {
    let vd = vd_f64(vec![1.1, 3.3], vec![false, false]);

    assert_eq!(vd.min_with_index(), (None, None));
}

#[test]
fn test_vd_min_with_index_sparse_valid() {
    let vd = vd_i64(
        vec![100, 5, 999, 1, 888, 777],
        vec![false, true, false, false, true, false],
    );

    let (val, idx) = vd.min_with_index();
    assert_eq!(val.unwrap(), 5);
    assert_eq!(idx.unwrap(), 1);
}