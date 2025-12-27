// === Imports ===
use crate::prelude::*;
use super::radixsort::radix_key::RadixKey;

// === Impl ===

impl<T> VectorData<T>
where
    T: PartialOrd + Default + Clone + Send + Sync + RadixKey + 'static,
{
    /// Unified entry: get sorted row indices.
    /// algo: "auto", "stable", "introsort", "heapsort", "radix"
    pub fn sort_indices(
        &self,
        ascending: bool,
        nulls_last: bool,
        algo: Option<&str>,
    ) -> Vec<usize> {
        let algo_name = algo.unwrap_or("auto").to_lowercase();

        match algo_name.as_str() {
            "auto" | "introsort" | "quicksort" => {
                self.sort_indices_introsort(ascending, nulls_last)
            }
            "stable" | "timsort" | "mergesort" => {
                self.sort_indices_stable(ascending, nulls_last)
            }
            "heapsort" => {
                self.sort_indices_heapsort(ascending, nulls_last)
            }
            "radix" => {
                if self.is_numeric() {
                    // numeric => safe to call radix
                    self.sort_indices_radixsort(ascending, nulls_last)
                } else {
                    self.sort_indices_introsort(ascending, nulls_last)
                }
            }
            _ => self.sort_indices_introsort(ascending, nulls_last),
        }
    }

    /// Internal fast sort — unchecked, returns raw VectorData<T>
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _sort(
        &self,
        ascending: bool,
        nulls_last: bool,
        algo: Option<&str>,
    ) -> Self {
        let idx = self.sort_indices(ascending, nulls_last, algo);
        self._reorder(&idx)
    }

    /// Non-mutating — return sorted VectorData
    pub fn sort(
        &self,
        ascending: bool,
        nulls_last: bool,
        algo: Option<&str>,
    ) -> ErrorResult<Self> {
        let idx = self.sort_indices(ascending, nulls_last, algo);
        self.reorder(&idx)
    }

    /// Internal fast in-place sort — unchecked, returns ()
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _sort_inplace(
        &mut self,
        ascending: bool,
        nulls_last: bool,
        algo: Option<&str>,
    ) {
        let idx = self.sort_indices(ascending, nulls_last, algo);
        self._reorder_inplace(&idx);
    }

    /// In-place sorting
    pub fn sort_inplace(
        &mut self,
        ascending: bool,
        nulls_last: bool,
        algo: Option<&str>,
    ) -> ErrorResult<()> {
        let idx = self.sort_indices(ascending, nulls_last, algo);
        self.reorder_inplace(&idx)
    }

    /// Alias: sort_by = reorder
    #[inline]
    pub fn sort_by(&self, indices: &[usize]) -> ErrorResult<Self> {
        self.reorder(indices)
    }

    /// Alias: sort_by_inplace = reorder_inplace
    #[inline]
    pub fn sort_by_inplace(&mut self, indices: &[usize]) -> ErrorResult<()> {
        self.reorder_inplace(indices)
    }
}