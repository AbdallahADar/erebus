// === Imports ===
use erebus_vector::prelude::*;

// === Tests ===

#[test]
fn test_vector_deepcopy_independence() {
    let v1 = Vector::full(10, 3);
    let v2 = v1.deepcopy();

    assert_eq!(v1, v2);

    let mut v2_mut = v2.clone();
    v2_mut.data[0] = 99;
    assert_ne!(v1.data[0], v2_mut.data[0]);
}

#[test]
fn test_vector_arc_deep_clone_independence() {
    let v = Vector::full(7, 4);
    let arc1 = v.arc_deep_clone();
    let arc2 = v.arc_deep_clone();

    assert_eq!(*arc1, *arc2);
    assert!(!Arc::ptr_eq(&arc1, &arc2)); // independent buffers
}

#[test]
fn test_vector_arc_shallow_clone_shares() {
    let v = Vector::full(3, 3);
    let arc1 = v.arc_deep_clone();
    let arc2 = Vector::arc_shallow_clone(&arc1);

    assert_eq!(Arc::strong_count(&arc1), 2);
    assert!(Arc::ptr_eq(&arc1, &arc2));
}