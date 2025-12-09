// === Imports ===
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cmp;
use once_cell::sync::Lazy;

// === Impls ===

pub struct AdaptiveChunker {
    avg_chunk: AtomicUsize,
    avg_time: AtomicUsize,   // microseconds
    max_cores: AtomicUsize,
}

impl AdaptiveChunker {

    pub fn new() -> Self {
        let cores = num_cpus::get_physical().clamp(1, 8);
        Self {
            avg_chunk: AtomicUsize::new(50_000),
            avg_time: AtomicUsize::new(200),
            max_cores: AtomicUsize::new(cores / 2),
        }
    }

    #[inline]
    pub fn set_max_cores(&self, n: usize) {
        self.max_cores.store(n, Ordering::Relaxed);
    }

    #[inline]
    pub fn suggest_chunk(&self, len: usize) -> usize {
        let cores = self.max_cores.load(Ordering::Relaxed);
        let base = cmp::max(len / (cores * 8).max(1), 1024);
        let prev = self.avg_chunk.load(Ordering::Relaxed);
        ((base + prev) / 2).clamp(1024, 1_000_000)
    }

    #[inline]
    pub fn record(&self, chunk: usize, micros: u128) {
        let prev_chunk = self.avg_chunk.load(Ordering::Relaxed);
        let prev_time = self.avg_time.load(Ordering::Relaxed);
        let new_chunk = ((prev_chunk + chunk) / 2).clamp(512, 2_000_000);
        let new_time = ((prev_time as u128 + micros) / 2) as usize;
        self.avg_chunk.store(new_chunk, Ordering::Relaxed);
        self.avg_time.store(new_time, Ordering::Relaxed);
    }
}

pub static ADAPTIVE_CHUNKER: Lazy<AdaptiveChunker> = Lazy::new(AdaptiveChunker::new);