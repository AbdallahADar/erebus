// === Imports ===
use crate::prelude::*;
use rayon::prelude::*;
use std::time::Instant;

// === Impl ===

impl<T> Vector<T>
where
    T: Clone + Send + Sync,
{
    /// Apply a unary function to produce a new `Vector<U>`.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_owned<U, F>(&self, f: F) -> Vector<U>
    where
        F: Fn(&T) -> U + Sync + Send,
        U: Clone + Send + Sync,
    {
        let n = self.data.len();
        let (use_parallel, chunk) = should_parallelize(n);
        let start = Instant::now();

        let data: Vec<U> = if use_parallel {
            self.data
                .par_chunks(chunk)
                .flat_map_iter(|chunk| chunk.iter().map(|x| f(x)))
                .collect()
        } else {
            let mut out = Vec::with_capacity(n);
            for val in &self.data {
                out.push(f(val));
            }
            out
        };

        if use_parallel {
            record_chunk_stats(n, start.elapsed().as_micros());
        }

        Vector { data }
    }

    /// Mutate elements in place.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_inplace<F>(&mut self, f: F)
    where
        F: Fn(&mut T) + Sync + Send,
    {
        let n = self.data.len();
        let (use_parallel, chunk) = should_parallelize(n);
        let start = Instant::now();

        if use_parallel {
            self.data.par_chunks_mut(chunk).for_each(|chunk| {
                chunk.iter_mut().for_each(|x| f(x));
            });
        } else {
            for val in &mut self.data {
                f(val);
            }
        }

        if use_parallel {
            record_chunk_stats(n, start.elapsed().as_micros());
        }

    }
}