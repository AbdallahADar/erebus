// === Imports ===
use crate::prelude::*;
use super::sift_down::sift_down;
use crate::vector::sort::Compare;

// === Impl ===

impl<T> Vector<T>
where
    T: PartialOrd + Clone + Default,
{
    /// Return sorted row indices using heapsort (unstable).
    pub(crate) fn sort_indices_heapsort(
        &self,
        ascending: bool,
    ) -> Vec<usize> {
        let n = self.data.len();
        if n <= 1 {
            return (0..n).collect();
        }

        // Comparator only needs &self.data and ascending
        let cmp = Compare::new(&self.data, ascending);

        // Initial heap = 0..n
        let mut heap: Vec<usize> = (0..n).collect();

        // Heapify (max-heap)
        let mut start = (n - 2) / 2;
        let mut end = n - 1;

        loop {
            sift_down(&cmp, &mut heap, start, end);
            if start == 0 {
                break;
            }
            start -= 1;
        }

        // Pop max → end repeatedly
        while end > 0 {
            heap.swap(0, end);
            end -= 1;
            sift_down(&cmp, &mut heap, 0, end);
        }

        heap
    }
}