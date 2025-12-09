use erebus_vector::prelude::*;
use std::any::type_name;

fn print_hashable<T: HashableValue>() {
    println!("HashableValue: {}", type_name::<T>());
}

#[test]
fn test_hashable_printouts() {
    // These should compile
    print_hashable::<i64>();
    print_hashable::<bool>();
    print_hashable::<String>();

    // These should NOT compile if HashableValue is correct
    // Comment them out, then try uncommenting to see whether compiler thinks they're allowed.

    print_hashable::<f64>();   // EXPECTED: compile error
//     print_hashable::<usize>(); // EXPECTED: compile error
    // print_hashable::<()>();    // EXPECTED: compile error
}