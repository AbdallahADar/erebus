// === Imports ===
use crate::prelude::*;
use super::Compare;

// === Impl ===

impl<T> VectorData<T>
where
    T: PartialOrd + Clone + Send + Sync + 'static,
{
    /// Fast unstable introsort (Rust’s sort_unstable_by)
    pub(crate) fn sort_indices_introsort(
        &self,
        ascending: bool,
        nulls_last: bool,
    ) -> Vec<usize> {

        let n = self.data.len();
        let mut idx: Vec<usize> = (0..n).collect();

        // Bring in Compare adapter
        let cmp = Compare::new(&self.data, &self.validity, ascending, nulls_last);

        idx.sort_unstable_by(|&i, &j| cmp.cmp(i, j));

        idx
    }

    /// Stable version using Rust's sort_by()
    pub(crate) fn sort_indices_stable(
        &self,
        ascending: bool,
        nulls_last: bool,
    ) -> Vec<usize> {

        let n = self.data.len();
        let mut idx: Vec<usize> = (0..n).collect();

        let cmp = Compare::new(&self.data, &self.validity, ascending, nulls_last);

        idx.sort_by(|&i, &j| cmp.cmp(i, j));

        idx
    }
}