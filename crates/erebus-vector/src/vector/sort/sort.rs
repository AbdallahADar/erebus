// === Imports ===
use crate::prelude::*;
use super::radixsort::radix_key::RadixKey;

// === Impl ===

impl<T> Vector<T>
where
    T: PartialOrd + Default + Clone + Send + Sync + RadixKey + 'static,
{
    /// Unified entry: get sorted row indices.
    /// algo: "auto", "stable", "introsort", "heapsort", "radix"
    pub fn sort_indices(
        &self,
        ascending: bool,
        algo: Option<&str>,
    ) -> Vec<usize> {
        let algo_name = algo.unwrap_or("auto").to_lowercase();

        match algo_name.as_str() {
            "auto" | "introsort" | "quicksort" => {
                self.sort_indices_introsort(ascending)
            }
            "stable" | "timsort" | "mergesort" => {
                self.sort_indices_stable(ascending)
            }
            "heapsort" => {
                self.sort_indices_heapsort(ascending)
            }
            "radix" => {
                if self.is_numeric() {
                    self.sort_indices_radixsort(ascending)
                } else {
                    self.sort_indices_introsort(ascending)
                }
            }
            _ => self.sort_indices_introsort(ascending),
        }
    }

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _sort(
        &self,
        ascending: bool,
        algo: Option<&str>,
    ) -> Self {
        let idx = self.sort_indices(ascending, algo);
        self._reorder(&idx)
    }

    /// Non-mutating — return a sorted Vector<T>
    pub fn sort(
        &self,
        ascending: bool,
        algo: Option<&str>,
    ) -> ErrorResult<Self> {
        let idx = self.sort_indices(ascending, algo);
        self.reorder(&idx)
    }

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _sort_inplace(
        &mut self,
        ascending: bool,
        algo: Option<&str>,
    ) {
        let idx = self.sort_indices(ascending, algo);
        self._reorder_inplace(&idx);
    }

    pub fn sort_inplace(
        &mut self,
        ascending: bool,
        algo: Option<&str>,
    ) -> ErrorResult<()> {
        let idx = self.sort_indices(ascending, algo);
        self.reorder_inplace(&idx)
    }

    #[inline]
    pub fn sort_by(&self, indices: &[usize]) -> ErrorResult<Self> {
        self.reorder(indices)
    }

    #[inline]
    pub fn sort_by_inplace(&mut self, indices: &[usize]) -> ErrorResult<()> {
        self.reorder_inplace(indices)
    }
}