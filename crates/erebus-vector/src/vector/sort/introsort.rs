// === Imports ===
use crate::prelude::*;
use super::Compare;

// === Impl ===

impl<T> Vector<T>
where
    T: PartialOrd + Clone + Send + Sync + 'static,
{
    pub(crate) fn sort_indices_introsort(
        &self,
        ascending: bool,
    ) -> Vec<usize> {

        let n = self.data.len();
        let mut idx: Vec<usize> = (0..n).collect();

        let cmp = Compare::new(&self.data, ascending);

        idx.sort_unstable_by(|&i, &j| cmp.cmp(i, j));
        idx
    }

    pub(crate) fn sort_indices_stable(
        &self,
        ascending: bool,
    ) -> Vec<usize> {

        let n = self.data.len();
        let mut idx: Vec<usize> = (0..n).collect();

        let cmp = Compare::new(&self.data, ascending);

        idx.sort_by(|&i, &j| cmp.cmp(i, j));
        idx
    }
}