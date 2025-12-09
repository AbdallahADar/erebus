// === Imports ===
use crate::prelude::*;
use std::cmp::Ordering;

// === Impl ===

pub struct Compare<'a, T> {
    data: &'a [T],
    ascending: bool,
}

impl<'a, T: PartialOrd> Compare<'a, T> {
    #[inline]
    pub fn new(
        data: &'a [T],
        ascending: bool,
    ) -> Self {
        Self { data, ascending }
    }

    #[inline]
    pub fn cmp(&self, i: usize, j: usize) -> Ordering {
        let ord = self.data[i]
            .partial_cmp(&self.data[j])
            .unwrap_or(Ordering::Equal);

        if self.ascending { ord } else { ord.reverse() }
    }
}