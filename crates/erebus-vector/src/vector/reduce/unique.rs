// === Imports ===
use crate::prelude::*;

// === Impl ===

#[inline]
pub fn n_unique_hash<T: Eq + Hash>(data: &[T]) -> usize {
    let mut set = HashSet::with_capacity(data.len());
    for v in data {
        set.insert(v);
    }
    set.len()
}

#[inline]
pub fn unique_hash<T: Clone + Eq + Hash>(data: &[T]) -> Vec<T> {
    let mut seen = HashSet::with_capacity(data.len());
    let mut out = Vec::new();

    for v in data {
        if seen.insert(v) {
            out.push(v.clone());
        }
    }
    out
}

impl Vector<i64> {
    #[inline]
    pub fn n_unique(&self) -> usize {
        n_unique_hash(&self.data)
    }

    #[inline]
    pub fn n_distinct(&self) -> usize {
        self.n_unique()
    }

    #[inline]
    pub fn unique(&self) -> Self {
        let out = unique_hash(&self.data);
        Vector { data: out }
    }
}

impl Vector<bool> {
    #[inline]
    pub fn n_unique(&self) -> usize {
        n_unique_hash(&self.data)
    }

    #[inline]
    pub fn n_distinct(&self) -> usize {
        self.n_unique()
    }

    #[inline]
    pub fn unique(&self) -> Self {
        let out = unique_hash(&self.data);
        Vector { data: out }
    }
}

impl Vector<String> {
    #[inline]
    pub fn n_unique(&self) -> usize {
        n_unique_hash(&self.data)
    }

    #[inline]
    pub fn n_distinct(&self) -> usize {
        self.n_unique()
    }

    #[inline]
    pub fn unique(&self) -> Self {
        let out = unique_hash(&self.data);
        Vector { data: out }
    }
}

impl Vector<f64> {
    #[inline]
    pub fn n_unique(&self) -> usize {
        let mut seen: HashSet<OrderedFloat<f64>> = HashSet::new();
        for &v in &self.data {
            seen.insert(OrderedFloat(v));
        }
        seen.len()
    }

    #[inline]
    pub fn n_distinct(&self) -> usize {
        self.n_unique()
    }

    #[inline]
    pub fn unique(&self) -> Self {
        let mut seen: HashSet<OrderedFloat<f64>> = HashSet::new();
        let mut out = Vec::new();

        for &v in &self.data {
            if seen.insert(OrderedFloat(v)) {
                out.push(v);
            }
        }

        Vector { data: out }
    }
}