// === Imports ===
use crate::prelude::*;
use rayon::prelude::*;
use std::time::Instant;

// === Impl ===

// Min, Max, Argmin, Argmax
// Median, Quantile, Any, All,
// Mode, value counts, n_unique,n_distinct,
// first, last, first_valid, last_valid, nth
// concat
// Weighted aggregators
// Window functions aka cumulative functions

impl<T> VectorData<T>
where
    T: Clone + Send + Sync,
{
    pub fn _reduce<R>(&self, mut reducer: R) -> R::Output
    where
        R: Reducer<T> + Clone,
    {
        let n = self.data.len();
        if n == 0 {
            return reducer.finalize(R::Acc::default());
        }

        let (use_parallel, chunk_size) = should_parallelize(n);
        let start = Instant::now();

        if use_parallel {

            let partials: Vec<R::Acc> = self
                .data
                .par_chunks(chunk_size)
                .enumerate()
                .map(|(chunk_idx, chunk)| {
                    let mut local_acc = R::Acc::default();
                    let mut local_reducer = reducer.clone();

                    let validity = &self.validity;
                    let start_i = chunk_idx * chunk_size;

                    for (i, value) in chunk.iter().enumerate() {
                        let idx = start_i + i;
                        let is_valid = unsafe { *validity.get_unchecked(idx) };
                        local_reducer.accumulate(&mut local_acc, value, is_valid);
                    }
                    local_acc
                })
                .collect();

            // Combine partials
            let mut final_acc = R::Acc::default();
            for p in partials {
                reducer.combine(&mut final_acc, p);
            }

            record_chunk_stats(n, start.elapsed().as_micros());
            reducer.finalize(final_acc)

        } else {

            let mut acc = R::Acc::default();
            for i in 0..n {
                let is_valid = unsafe { *self.validity.get_unchecked(i) };
                reducer.accumulate(&mut acc, &self.data[i], is_valid);
            }
            reducer.finalize(acc)
        }
    }

    pub fn _reduce_indexed<R>(&self, mut reducer: R) -> R::Output
    where
        R: ReducerIndexed<T> + Send + Sync + Clone,
    {
        let n = self.data.len();
        if n == 0 {
            return reducer.finalize(R::Acc::default());
        }

        let (use_parallel, chunk_size) = should_parallelize(n);
        let start = Instant::now();

        if use_parallel {

            let partials: Vec<R::Acc> = self
                .data
                .par_chunks(chunk_size)
                .enumerate()
                .map(|(chunk_idx, chunk)| {
                    let mut local_acc = R::Acc::default();
                    let mut local_reducer = reducer.clone();

                    let validity = &self.validity;
                    let start_i = chunk_idx * chunk_size;

                    for (i, value) in chunk.iter().enumerate() {
                        let idx = start_i + i;
                        let is_valid = unsafe { *validity.get_unchecked(idx) };
                        local_reducer.accumulate(&mut local_acc, value, is_valid, idx);
                    }

                    local_acc
                })
                .collect();

            let mut final_acc = R::Acc::default();
            for p in partials {
                reducer.combine(&mut final_acc, p);
            }

            record_chunk_stats(n, start.elapsed().as_micros());
            reducer.finalize(final_acc)

        } else {

            let mut acc = R::Acc::default();
            for idx in 0..n {
                let is_valid = unsafe { *self.validity.get_unchecked(idx) };
                reducer.accumulate(&mut acc, &self.data[idx], is_valid, idx);
            }
            reducer.finalize(acc)
        }
    }
}