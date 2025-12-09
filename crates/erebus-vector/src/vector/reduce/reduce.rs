// === Imports ===
use crate::prelude::*;
use rayon::prelude::*;
use std::time::Instant;

// === Impl ===

impl<T> Vector<T>
where
    T: Clone + Send + Sync,
{
    pub fn _reduce<R>(&self, mut reducer: R) -> R::Output
    where
        R: Reducer<T> + Send + Sync + Clone,
    {
        let n = self.data.len();
        if n == 0 {
            return reducer.finalize(R::Acc::default());
        }

        let (use_parallel, chunk) = should_parallelize(n);
        let start = Instant::now();

        if use_parallel {
            // Parallel version
            let partials: Vec<R::Acc> = self
                .data
                .par_chunks(chunk)
                .map(|chunk| {
                    let mut local_acc = R::Acc::default();
                    let mut local_reducer = reducer.clone();
                    for value in chunk {
                        local_reducer.accumulate(&mut local_acc, value, true);
                    }
                    local_acc
                })
                .collect();

            let mut final_acc = R::Acc::default();
            let mut final_reducer = reducer;
            for p in partials {
                final_reducer.combine(&mut final_acc, p);
            }

            record_chunk_stats(n, start.elapsed().as_micros());
            final_reducer.finalize(final_acc)

        } else {
            // Serial version
            let mut acc = R::Acc::default();
            for v in &self.data {
                reducer.accumulate(&mut acc, v, true);
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

                    let base = chunk_idx * chunk_size;

                    for (i, v) in chunk.iter().enumerate() {
                        // ORDER MUST MATCH THE TRAIT:
                        // accumulate(acc, value: &T, is_valid, idx)
                        local_reducer.accumulate(
                            &mut local_acc,
                            v,
                            true,           // Vector has no nulls
                            base + i,       // global index
                        );
                    }
                    local_acc
                })
                .collect();

            let mut final_acc = R::Acc::default();
            let mut final_reducer = reducer;

            for p in partials {
                final_reducer.combine(&mut final_acc, p);
            }

            record_chunk_stats(n, start.elapsed().as_micros());
            final_reducer.finalize(final_acc)

        } else {
            let mut acc = R::Acc::default();

            for (idx, v) in self.data.iter().enumerate() {
                reducer.accumulate(
                    &mut acc,
                    v,
                    true,   // always valid
                    idx,
                );
            }

            reducer.finalize(acc)
        }
    }

}