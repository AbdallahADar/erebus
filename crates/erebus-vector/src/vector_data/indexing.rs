// === Imports ===
use crate::prelude::*;

// === Impl ===

impl<T: Clone + Send + Sync + 'static> VectorData<T> {

    /// Returns the first element (valid only). Null → None.
    #[inline]
    pub fn first(&self) -> Option<T> {
        if self.data.is_empty() { return None; }
        if self.validity[0] { Some(self.data[0].clone()) } else { None }
    }

    pub fn first_valid(&self) -> Option<T> {
        for i in 0..self.data.len() {
            if self.validity[i] {
                return Some(self.data[i].clone());
            }
        }
        None
    }

    /// Returns last element (valid only). Null → None.
    #[inline]
    pub fn last(&self) -> Option<T> {
        let n = self.data.len();
        if n == 0 { return None; }
        let i = n - 1;
        if self.validity[i] { Some(self.data[i].clone()) } else { None }
    }

    pub fn last_valid(&self) -> Option<T> {
        for i in (0..self.data.len()).rev() {
            if self.validity[i] {
                return Some(self.data[i].clone());
            }
        }
        None
    }

    /// Returns Option<T>, handling null and out-of-bounds.
    #[inline]
    pub fn get(&self, i: usize) -> Option<T> {
        if i >= self.data.len() { return None; }
        if self.validity[i] { Some(self.data[i].clone()) } else { None }
    }

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _get(&self, i: usize) -> Option<T> {
        if unsafe { *self.validity.get_unchecked(i) } {
            Some(unsafe { self.data.get_unchecked(i).clone() })
        } else {
            None
        }
    }

    /// Returns nth element (valid only). Null → None.
    #[inline]
    pub fn nth(&self, n: isize) -> Option<T> {
        let len = self.data.len();
        if len == 0 { return None; }

        let idx = if n < 0 {
            (len as isize + n) as usize
        } else {
            n as usize
        };

        if idx >= len { return None; }

        if self.validity[idx] {
            Some(self.data[idx].clone())
        } else {
            None
        }
    }

    /// Internal unsafe fast version (no bounds checks).
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _take(&self, idx: &[usize]) -> Self {
        let m = idx.len();
        let mut out = Vec::with_capacity(m);
        unsafe { out.set_len(m); }

        let mut out_validity = bitvec![0; m];

        for (j, &i) in idx.iter().enumerate() {
            unsafe {
                *out.get_unchecked_mut(j) = self.data.get_unchecked(i).clone();
                out_validity.set(j, *self.validity.get_unchecked(i));
            }
        }
        VectorData {data: out, validity: out_validity,}
    }

    #[inline]
    pub fn take(&self, idx: &[usize]) -> ErrorResult<Self> {
        let n = self.data.len();

        for &i in idx {
            if i >= n {
                return Err(ErebusError::IndexOutOfBounds {
                    index: i,
                    size: n,
                });
            }
        }
        Ok(self._take(idx))
    }

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _bool_index(&self, mask: &[bool]) -> Self {
        let n = self.data.len();
        debug_assert_eq!(mask.len(), n);

        let mut out = Vec::<T>::with_capacity(n);
        let mut out_validity = bitvec![0; n];

        unsafe {
            let mut j = 0usize;
            let out_ptr = out.as_mut_ptr();

            for i in 0..n {
                if *mask.get_unchecked(i) {
                    // copy data
                    std::ptr::write(out_ptr.add(j), self.data.get_unchecked(i).clone());
                    // copy validity
                    out_validity.set_unchecked(j, *self.validity.get_unchecked(i));
                    j += 1;
                }
            }

            out.set_len(j);
            out_validity.truncate(j);
        }

        VectorData { data: out, validity: out_validity }
    }

    /// Public safe version
    #[inline]
    pub fn bool_index(&self, mask: &[bool]) -> ErrorResult<Self> {
        let n = self.data.len();
        if mask.len() != n {
            return Err(ErebusError::LengthMismatch {
                expected: n,
                found: mask.len(),
            });
        }
        Ok(self._bool_index(mask))
    }

    /// Returns an owned slice copy in range `[start, end)`.
    /// Clamps range to valid bounds and preserves validity.
    #[inline]
    pub fn slice(&self, start: usize, end: usize) -> Self {
        if start >= end || start >= self.data.len() {
            return Self {
                data: Vec::new(),
                validity: bitvec![0; 0],
            };
        }

        let end = end.min(self.data.len());
        Self {
            data: self.data[start..end].to_vec(),
            validity: self.validity[start..end].to_bitvec(),
        }
    }

    /// Returns a zero-copy borrowed view over the range `[start, end)`.
    /// Performs no allocations; safe if parent `VectorData` remains alive.
    #[inline]
    pub fn slice_view(&self, start: usize, end: usize) -> VectorDataView<'_, T> {
        let len = self.data.len();
        if start >= end || start >= len {
            return VectorDataView {
                data: &[],
                validity: BitSlice::empty(),
            };
        }

        let end = end.min(len);
        VectorDataView {
            data: &self.data[start..end],
            validity: &self.validity[start..end],
        }
    }
}

impl VectorData<bool> {
    /// Return indices where the value is true.
    pub fn arg_true(&self) -> Vec<usize> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if v { Some(i) } else { None })
            .collect()
    }

    /// Return indices where the value is false.
    pub fn arg_false(&self) -> Vec<usize> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(i, &v)| if !v { Some(i) } else { None })
            .collect()
    }
}