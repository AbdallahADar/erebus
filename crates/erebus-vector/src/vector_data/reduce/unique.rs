// === Imports ===
use crate::prelude::*;

// === Impl ===

// Big lesson learned:
// We cannot impl on type T a generic function
// and then impl same named function on a specific type
// We even create a trait HashableValue type to limit what the generic T impl used
// But apparently rust sees generic T as encompassing during definition
// and while the trait worked in theory because it wouldn't compile on f64, it still
// did not allow specialized definition of f64 using same named functions.

/// Generic hash-based distinct count for Eq+Hash types (i64, bool, String, …)
fn n_unique_hash<T: Eq + Hash + Clone>(data: &[T], validity: &BitSlice) -> usize {
    let mut set = HashSet::new();
    for i in 0..data.len() {
        if unsafe { *validity.get_unchecked(i) } {
            set.insert(unsafe { data.get_unchecked(i).clone() });
        }
    }
    set.len()
}

fn unique_hash<T: Eq + Hash + Clone>(data: &[T], validity: &BitSlice) -> VectorData<T> {
    let n = data.len();
    let mut seen = HashSet::with_capacity(n);
    let mut out = Vec::new();
    let mut n_out = 0usize;

    for i in 0..n {
        let is_valid = unsafe { *validity.get_unchecked(i) };
        if is_valid {
            let v = unsafe { data.get_unchecked(i).clone() };
            if seen.insert(v.clone()) {
                out.push(v);
                n_out += 1usize;
            }
        }
    }
    VectorData {
        data: out,
        validity: bitvec![1; n_out],
    }
}

impl VectorData<i64> {
    #[inline]
    pub fn n_unique(&self) -> usize {
        n_unique_hash(&self.data, &self.validity)
    }

    #[inline]
    pub fn n_distinct(&self) -> usize {
        self.n_unique()
    }

    #[inline]
    pub fn unique(&self) -> Self {
        unique_hash(&self.data, &self.validity)
    }
}

impl VectorData<bool> {
    #[inline]
    pub fn n_unique(&self) -> usize {
        n_unique_hash(&self.data, &self.validity)
    }

    #[inline]
    pub fn n_distinct(&self) -> usize {
        self.n_unique()
    }

    #[inline]
    pub fn unique(&self) -> Self {
        unique_hash(&self.data, &self.validity)
    }
}

impl VectorData<String> {
    #[inline]
    pub fn n_unique(&self) -> usize {
        n_unique_hash(&self.data, &self.validity)
    }

    #[inline]
    pub fn n_distinct(&self) -> usize {
        self.n_unique()
    }

    #[inline]
    pub fn unique(&self) -> Self {
        unique_hash(&self.data, &self.validity)
    }
}

impl VectorData<f64> {
    #[inline]
    pub fn n_unique(&self) -> usize {
        let mut seen: HashSet<OrderedFloat<f64>> = HashSet::new();
        for i in 0..self.data.len() {
            if unsafe { *self.validity.get_unchecked(i) } {
                seen.insert(OrderedFloat(unsafe { *self.data.get_unchecked(i) }));
            }
        }
        seen.len()
    }

    #[inline]
    pub fn n_distinct(&self) -> usize {
        self.n_unique()
    }

    #[inline]
    pub fn unique(&self) -> Self {
        let n = self.data.len();
        let mut seen: HashSet<OrderedFloat<f64>> = HashSet::with_capacity(n);
        let mut out = Vec::new();
        let mut n_out = 0usize;

        for i in 0..n {
            if unsafe { *self.validity.get_unchecked(i) } {
                let v = unsafe { *self.data.get_unchecked(i) };
                if seen.insert(OrderedFloat(v)) {
                    out.push(v);
                    n_out += 1usize;
                }
            }
        }

        VectorData {
            data: out,
            validity: bitvec![1; n_out],
        }
    }
}