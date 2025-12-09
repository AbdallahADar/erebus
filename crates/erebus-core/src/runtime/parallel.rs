// === Imports ===
use super::adaptive_chunker::ADAPTIVE_CHUNKER;
use std::sync::atomic::{AtomicBool, Ordering};

// === Impls ===

/// Global flag that determines whether parallel execution is allowed.
/// Default: **true**
/// Meaning: Erebus will use rayon parallel kernels where appropriate.
pub static ENABLE_PARALLEL: AtomicBool = AtomicBool::new(true);

/// Enable parallel execution globally.
/// All vector/table kernels will use parallel paths when possible.
#[inline]
pub fn enable_parallel() {
    ENABLE_PARALLEL.store(true, Ordering::Relaxed);
}

/// Disable parallel execution globally.
/// Forces all operations to run single-threaded.
#[inline]
pub fn disable_parallel() {
    ENABLE_PARALLEL.store(false, Ordering::Relaxed);
}

/// Check if parallel execution is currently enabled.
#[inline]
pub fn is_parallel_enabled() -> bool {
    ENABLE_PARALLEL.load(Ordering::Relaxed)
}

/// Decides whether to parallelize based on adaptive chunk estimator.
/// Returns `(use_parallel, suggested_chunk_size)`.
#[inline]
pub fn should_parallelize(len: usize) -> (bool, usize) {

    // Global override: user disabled parallelism
    if !is_parallel_enabled() { return (false, 0); }

    // Ask adaptive chunker for a recommended chunk size
    let chunk = ADAPTIVE_CHUNKER.suggest_chunk(len);

    // Heuristic: only parallelize if we have enough work
    // If `len < 2 * chunk`, parallel overhead likely exceeds benefits.
    // If `len >= 2 * chunk`, parallel is worthwhile.
    (len >= chunk * 2, chunk)
}

/// Records the duration of a chunked operation (microseconds).
#[inline]
pub fn record_chunk_stats(len: usize, micros: u128) {
    ADAPTIVE_CHUNKER.record(len, micros);
}