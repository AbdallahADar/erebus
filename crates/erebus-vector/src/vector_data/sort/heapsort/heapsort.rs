// === Imports ===
use crate::prelude::*;
use super::sift_down::sift_down;
use crate::vector_data::sort::Compare;

// === Impl ===

/// Heapsort index-based sorting for VectorData
impl<T> VectorData<T>
where
    T: PartialOrd + Clone + Default,
{
    /// Return sorted row indices using heapsort (unstable).
    pub(crate) fn sort_indices_heapsort(
        &self,
        ascending: bool,
        nulls_last: bool,
    ) -> Vec<usize> {
        let n = self.len();
        if n <= 1 {
            return (0..n).collect();
        }

        // Build comparator
        let cmp = Compare::new(&self.data, &self.validity, ascending, nulls_last);

        // Initial heap = 0..n
        let mut heap: Vec<usize> = (0..n).collect();

        // Heapify (build max-heap)
        let mut start = (n - 2) / 2; // last non-leaf
        let end = n - 1;

        loop {
            sift_down(&cmp, &mut heap, start, end);
            if start == 0 {
                break;
            }
            start -= 1;
        }

        // Repeatedly pop max to end
        let mut end = end;
        while end > 0 {
            heap.swap(0, end);
            end -= 1;
            sift_down(&cmp, &mut heap, 0, end);
        }

        heap
    }
}