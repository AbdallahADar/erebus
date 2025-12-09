// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

#[test]
fn test_vector_data_deepcopy_independence() {
    let data = vec![10, 20, 30];
    let validity = bitvec![1, 0, 1];
    let v1 = VectorData::from_vec(data.clone(), validity.clone()).unwrap();

    let v2 = v1.deepcopy();
    assert_eq!(v1, v2);

    // Modify v2 to confirm v1 unaffected (true independence)
    let mut v2_mut = v2.clone();
    v2_mut.data[0] = 99;
    assert_ne!(v1.data[0], v2_mut.data[0]);
}

#[test]
fn test_vector_data_arc_deep_clone_independence() {
    let v1 = VectorData::full(5, 3);
    let arc1 = v1.arc_deep_clone();

    // The deep Arc clone produces a distinct allocation
    let arc2 = v1.arc_deep_clone();
    assert_eq!(*arc1, *arc2);
    assert!(!Arc::ptr_eq(&arc1, &arc2)); // ensure different Arc allocations
}

#[test]
fn test_vector_data_arc_shallow_clone_shares() {
    let v = VectorData::full(42, 2);
    let arc1 = v.arc_deep_clone(); // one Arc to start
    let arc2 = VectorData::arc_shallow_clone(&arc1);

    assert_eq!(Arc::strong_count(&arc1), 2);
    assert!(Arc::ptr_eq(&arc1, &arc2)); // same memory address
}