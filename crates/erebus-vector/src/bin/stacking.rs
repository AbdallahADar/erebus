// Deciding what approach to use for the `stack` function
// `stack` function concatenates the struct from a vector of structs
// stack_fast approach is the fatest so we go with that

use std::time::Instant;
use std::ptr;

/// Simulates a lightweight Vector-like structure
#[derive(Clone)]
struct SimpleVector {
    data: Vec<i64>,
}

// --- Safe extend_from_slice version ---
fn stack_extend(vectors: &[&SimpleVector]) -> SimpleVector {
    let total_len: usize = vectors.iter().map(|v| v.data.len()).sum();
    let mut data = Vec::with_capacity(total_len);

    for v in vectors {
        data.extend_from_slice(&v.data);
    }

    SimpleVector { data }
}

// --- Safe flat_map version ---
fn stack_flatmap(vectors: &[&SimpleVector]) -> SimpleVector {
    let data: Vec<i64> = vectors.iter().flat_map(|v| v.data.iter().cloned()).collect();
    SimpleVector { data }
}

// --- Unsafe ptr-copy version ---
unsafe fn stack_fast(vectors: &[&SimpleVector]) -> SimpleVector {
    let total_len: usize = vectors.iter().map(|v| v.data.len()).sum();
    let mut data = Vec::with_capacity(total_len);

    let mut ptr = data.as_mut_ptr();
    for v in vectors {
        let len = v.data.len();
        ptr::copy_nonoverlapping(v.data.as_ptr(), ptr, len);
        ptr = ptr.add(len);
    }
    data.set_len(total_len);

    SimpleVector { data }
}

// --- Helper to create test data ---
fn make_vectors(num_vectors: usize, len_each: usize) -> Vec<SimpleVector> {
    (0..num_vectors)
        .map(|i| SimpleVector {
            data: (0..len_each)
                .map(|x| x as i64 + i as i64)
                .collect(),
        })
        .collect()
}

// --- Timing helper ---
fn time_it<F: FnOnce() -> SimpleVector>(label: &str, f: F) {
    let start = Instant::now();
    let result = f();
    let dur = start.elapsed();
    println!(
        "{:<15} | len = {:>8} | time = {:>8.3?}",
        label,
        result.data.len(),
        dur
    );
}

// --- Main ---
fn main() {
    let num_vectors = 1000;   // vary this to test scaling
    let len_each = 100000;      // elements per vector

    let all_vecs: Vec<SimpleVector> = make_vectors(num_vectors, len_each);
    let refs: Vec<&SimpleVector> = all_vecs.iter().collect();

    println!("=== Vector stack performance test ===");
    println!("Vectors: {} × length {}\n", num_vectors, len_each);

    // Safe default
    time_it("stack_extend", || stack_extend(&refs));

    // Flatten variant
    time_it("stack_flatmap", || stack_flatmap(&refs));

    // Unsafe fast path
    time_it("stack_fast", || unsafe { stack_fast(&refs) });
}