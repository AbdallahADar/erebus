// === Imports ===

use crate::prelude::*;

// === Types ===

/// Vector-backed data with validity bitmap (Dense only for now).
#[derive(Debug, Clone, PartialEq)]
pub struct VectorData<T> {
    pub data: Vec<T>,      // All rows, including placeholders for nulls
    pub validity: BitVec,  // 1 = valid, 0 = null
}

// === Impl ===

impl<T: Clone + Default> VectorData<T> {

    /// Create an empty VectorData (no values, empty bitmap).
    #[inline]
    pub fn empty() -> Self {
        Self {
            data: Vec::new(),
            validity: BitVec::new(),
        }
    }

    /// Alias of `empty`
    pub fn new() -> Self { Self::empty() }

    #[inline]
    pub fn full(value: T, n: usize) -> Self {
        Self {
            data: vec![value; n],
            validity: bitvec![1; n], // all valid
        }
    }

    /// # Safety
    /// This function performs no bounds checks or validation.
    /// Intended for internal fast-path usage.
    /// Enabled only under the `internal` feature.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _from_vec(data: Vec<T>, validity: BitVec) -> Self {
        debug_assert_eq!(data.len(), validity.len(), "Validity length mismatch");
        Self { data, validity }
    }

    /// Creates a new [`VectorData`] safely.
    /// Returns an error if data and validity lengths mismatch.
    #[inline]
    pub fn from_vec(data: Vec<T>, validity: BitVec) -> ErrorResult<Self> {
        if data.len() != validity.len() {
            return Err(ErebusError::LengthMismatch {
                expected: data.len(),
                found: validity.len(),
            });
        }
        Ok(Self::_from_vec(data, validity))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Get null count in vector data
    #[inline]
    pub fn null_count(&self) -> usize {
        self.validity.count_zeros()
    }

    /// Track memory usage
    pub fn memory_usage(&self) -> usize {
        std::mem::size_of_val(&self.data)
            + self.data.capacity() * std::mem::size_of::<T>()
            + self.validity.as_raw_slice().len() * std::mem::size_of::<usize>()
    }

}

impl<T: Clone> From<Vector<T>> for VectorData<T> {
    fn from(v: Vector<T>) -> Self {
        let len = v.data.len();
        Self {
            data: v.data,
            validity: bitvec![1; len],
        }
    }
}

impl<T: std::fmt::Display> VectorData<T> {
    pub fn print(&self) -> String {
        let mut out = String::new();
        for (i, val) in self.data.iter().enumerate() {
            if self.validity.get(i).map(|b| *b).unwrap_or(true) {
                out.push_str(&format!("{}, ", val));
            } else {
                out.push_str("None, ");
            }
        }
        if out.ends_with(", ") {
            out.truncate(out.len() - 2); // remove trailing comma
        }
        format!("[{}]", out)
    }
}

impl<'a, T: 'a> Viewable<'a, T> for VectorData<T> {
    type ViewType = VectorDataView<'a, T>;

    #[inline]
    fn view(&'a self) -> Self::ViewType {
        VectorDataView {
            data: &self.data,
            validity: &self.validity,
        }
    }
}