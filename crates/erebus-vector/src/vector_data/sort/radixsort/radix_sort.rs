// === Imports ===
use crate::prelude::*;
use super::radix_key::RadixKey;

// === Impl ===

impl<T> VectorData<T>
where
    T: RadixKey + Default + PartialOrd + Clone + Send + Sync + 'static,
{
    /// Radix sort row indices (only works for numeric types).
    pub(crate) fn sort_indices_radixsort(
        &self,
        ascending: bool,
        nulls_last: bool,
    ) -> Vec<usize>
    {
        let n = self.len();
        if n <= 1 {
            return (0..n).collect();
        }

        // Separate valid and null rows
        let mut valid: Vec<usize> = Vec::with_capacity(n);
        let mut nulls: Vec<usize> = Vec::new();

        for i in 0..n {
            if self.validity[i] {
                valid.push(i);
            } else {
                nulls.push(i);
            }
        }

        // Edge case: all nulls OR no valid rows
        if valid.is_empty() {
            return if nulls_last {
                nulls
            } else {
                nulls
            };
        }

        // Build radix keys
        let mut keys: Vec<_> = valid
            .iter()
            .map(|&i| T::to_radix_key(self.data[i].clone()))
            .collect();

        let mut tmp = vec![0usize; valid.len()];

        const BITS_PER_PASS: usize = 8;
        const RADIX: usize = 1 << BITS_PER_PASS;
        let passes = (std::mem::size_of::<u64>() * 8) / BITS_PER_PASS;

        // Radix passes
        for pass in 0..passes {
            let shift = pass * BITS_PER_PASS;
            let mut count = [0usize; RADIX];

            // Histogram
            for &k in &keys {
                count[T::extract_byte(k, shift)] += 1;
            }

            // Prefix sum
            let mut sum = 0usize;
            for c in count.iter_mut() {
                let v = *c;
                *c = sum;
                sum += v;
            }

            // Reorder indices into tmp
            for (idx, &k) in keys.iter().enumerate() {
                let b = T::extract_byte(k, shift);
                let pos = count[b];
                tmp[pos] = valid[idx];
                count[b] = pos + 1;
            }

            // Copy tmp → valid
            valid.clone_from_slice(&tmp);

            // Rebuild keys for next pass
            for (j, &i) in valid.iter().enumerate() {
                keys[j] = T::to_radix_key(self.data[i].clone());
            }
        }

        // If descending — reverse stable output
        if !ascending { valid.reverse(); }

        // Merge nulls
        let mut out = Vec::with_capacity(n);
        if nulls_last {
            out.extend(valid.into_iter());
            out.extend(nulls.into_iter());
        } else {
            out.extend(nulls.into_iter());
            out.extend(valid.into_iter());
        }

        out
    }
}