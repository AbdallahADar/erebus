// === Imports ===
use crate::prelude::*;
use rayon::prelude::*;
use std::time::Instant;

// === Impl ===

impl<T> VectorData<T>
where
    T: Clone + Send + Sync,
{
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_owned<U, F>(&self, f: F) -> VectorData<U>
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
            self.data.iter().map(|x| f(x)).collect()
        };

        if use_parallel {
            record_chunk_stats(n, start.elapsed().as_micros());
        }

        VectorData {
            data,
            validity: self.validity.clone(),
        }
    }

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_inplace<F>(&mut self, mut f: F)
    where
        F: Fn(&mut T) + Sync + Send,
    {
        let n = self.data.len();
        let (use_parallel, chunk) = should_parallelize(n);
        let start = Instant::now();

        if use_parallel {
            self.data
                .par_chunks_mut(chunk)
                .for_each(|chunk| chunk.iter_mut().for_each(|x| f(x)));
        } else {
            self.data.iter_mut().for_each(|x| f(x));
        }

        if use_parallel {
            record_chunk_stats(n, start.elapsed().as_micros());
        }
    }

    /// Apply a unary function returning `(value, is_valid)` and
    /// merge the result with the existing validity bitmap.
    /// Produces a new [`VectorData<U>`].
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn map_unary_owned_with_validity<U, F>(
        &self,
        f: F,
    ) -> VectorData<U>
    where
        F: Fn(&T) -> (U, bool) + Sync + Send,
        U: Clone + Default + Send + Sync,
    {
        let n = self.data.len();
        let (use_parallel, chunk) = should_parallelize(n);
        let start = Instant::now();

        // --- Parallel deterministic mode ---
        if use_parallel {
            let (data, new_valid_flags): (Vec<U>, Vec<bool>) = self
                .data
                .par_chunks(chunk)
                .flat_map_iter(|chunk| chunk.iter().map(|x| f(x)))
                .unzip();

            // Merge validity bitmaps
            let validity: BitVec = self
                .validity
                .iter()
                .zip(new_valid_flags.iter())
                .map(|(orig, new)| *orig && *new)
                .collect();

            record_chunk_stats(n, start.elapsed().as_micros());
            return VectorData { data, validity };
        }

        // --- Sequential version ---
        let mut data = vec![U::default(); n];
        let mut validity = self.validity.clone();
        for (i, val) in self.data.iter().enumerate() {
            let (v, is_valid) = f(val);
            data[i] = v;
            if !is_valid {
                // safe, already allocated
                unsafe { validity.set_unchecked(i, false); }
            }
        }

        VectorData { data, validity }
    }

}