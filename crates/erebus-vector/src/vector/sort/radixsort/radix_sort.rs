// === Imports ===
use crate::prelude::*;
use super::radix_key::RadixKey;

// === Impl ===

impl<T> Vector<T>
where
    T: RadixKey + Default + PartialOrd + Clone + Send + Sync + 'static,
{
    /// Radix sort row indices for Vector (no nulls).
    /// Works only for numeric RadixKey types.
    pub(crate) fn sort_indices_radixsort(
        &self,
        ascending: bool,
    ) -> Vec<usize> {

        let n = self.len();
        if n <= 1 {
            return (0..n).collect();
        }

        // Initial indices
        let mut idx: Vec<usize> = (0..n).collect();

        // Build initial radix keys
        let mut keys: Vec<_> = idx
            .iter()
            .map(|&i| T::to_radix_key(self.data[i].clone()))
            .collect();

        let mut tmp = vec![0usize; n];

        const BITS: usize = 8;
        const RADIX: usize = 1 << BITS;
        let passes = (std::mem::size_of::<u64>() * 8) / BITS;

        for pass in 0..passes {
            let shift = pass * BITS;
            let mut count = [0usize; RADIX];

            // Histogram
            for &k in &keys {
                count[T::extract_byte(k, shift)] += 1;
            }

            // Prefix-sum
            let mut sum = 0usize;
            for c in count.iter_mut() {
                let v = *c;
                *c = sum;
                sum += v;
            }

            // Reorder into tmp
            for (j, &k) in keys.iter().enumerate() {
                let b = T::extract_byte(k, shift);
                let pos = count[b];
                tmp[pos] = idx[j];
                count[b] = pos + 1;
            }

            idx.clone_from_slice(&tmp);

            // Rebuild keys
            for (j, &i) in idx.iter().enumerate() {
                keys[j] = T::to_radix_key(self.data[i].clone());
            }
        }

        // Descending: reverse result
        if !ascending {
            idx.reverse();
        }

        idx
    }
}