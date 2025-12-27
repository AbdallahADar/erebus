// === Imports ===
use crate::prelude::*;

// === Impl ===

impl<T: Clone + Send + Sync + 'static> Vector<T> {

    #[inline]
    pub fn first(&self) -> Option<T> {
        if self.data.is_empty() { None }
        else { Some(self.data[0].clone()) }
    }

    #[inline]
    pub fn last(&self) -> Option<T> {
        let n = self.data.len();
        if n == 0 { None }
        else { Some(self.data[n - 1].clone()) }
    }

    #[inline]
    pub fn get(&self, i: usize) -> Option<T> {
        self.data.get(i).cloned()
    }

    #[inline]
    pub fn nth(&self, n: isize) -> Option<T> {
        let len = self.data.len();
        if len == 0 { return None; }

        let idx = if n < 0 {
            (len as isize + n) as usize
        } else {
            n as usize
        };

        self.data.get(idx).cloned()
    }

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _take(&self, idx: &[usize]) -> Self {
        let m = idx.len();

        let mut out = Vec::with_capacity(m);
        unsafe { out.set_len(m); }

        unsafe {
            let out_ptr: *mut T = out.as_mut_ptr();
            for (j, &i) in idx.iter().enumerate() {
                std::ptr::write(out_ptr.add(j), self.data.get_unchecked(i).clone());
            }
        }

        Vector { data: out }
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

        let mut out = Vec::<T>::with_capacity(n);

        unsafe {
            let out_ptr = out.as_mut_ptr();
            let mut j = 0usize;

            for i in 0..n {
                if *mask.get_unchecked(i) {
                    std::ptr::write(out_ptr.add(j), self.data.get_unchecked(i).clone());
                    j += 1;
                }
            }
            out.set_len(j);
        }

        Vector { data: out }
    }

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

    /// Returns an owned copy of a slice in range `[start, end)`.
    #[inline]
    pub fn slice(&self, start: usize, end: usize) -> Self {
        if start >= end || start >= self.data.len() {
            return Self { data: Vec::new() };
        }

        let end = end.min(self.data.len());
        Self {
            data: self.data[start..end].to_vec(),
        }
    }

    /// Returns a zero-copy borrowed view of the range `[start, end)`.
    #[inline]
    pub fn slice_view(&self, start: usize, end: usize) -> VectorView<'_, T> {
        let len = self.data.len();
        if start >= end || start >= len {
            return VectorView { data: &[] };
        }

        let end = end.min(len);
        VectorView { data: &self.data[start..end] }
    }
}

impl Vector<bool> {
    /// Return indices where the value is true.
    pub fn arg_true(&self) -> Vec<usize> {
        self.data.iter()
            .enumerate()
            .filter_map(|(i, &v)| if v { Some(i) } else { None })
            .collect()
    }

    /// Return indices where the value is false.
    pub fn arg_false(&self) -> Vec<usize> {
        self.data.iter()
            .enumerate()
            .filter_map(|(i, &v)| if !v { Some(i) } else { None })
            .collect()
    }
}