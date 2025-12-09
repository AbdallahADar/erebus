// === Imports ===
use crate::prelude::*;
use crate::vector::sort::Compare;

// === Impl ===

/// Sift-down helper for heapsort.
/// Uses Compare::cmp(i, j) to compare underlying values.
#[inline(never)]
pub(crate) fn sift_down<T>(
    cmp: &Compare<'_, T>,
    heap: &mut [usize],
    start: usize,
    end: usize,
) where
    T: PartialOrd,
{
    let mut root = start;

    loop {
        let left = 2 * root + 1;
        if left > end {
            break;
        }

        let mut swap_idx = root;

        // Compare left child
        if cmp.cmp(heap[swap_idx], heap[left]) == Ordering::Less {
            swap_idx = left;
        }

        // Compare right child
        let right = left + 1;
        if right <= end && cmp.cmp(heap[swap_idx], heap[right]) == Ordering::Less {
            swap_idx = right;
        }

        if swap_idx == root {
            return;
        }

        heap.swap(root, swap_idx);
        root = swap_idx;
    }
}