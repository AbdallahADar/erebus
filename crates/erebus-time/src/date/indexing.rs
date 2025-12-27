// === Imports ===
use crate::prelude::*;

// === Impl ===

impl DateVector {

    /// Returns `Some(Date)` if valid, `None` if null.
    /// Errors if index is out of bounds.
    #[inline]
    pub fn get(&self, i: usize) -> Option<Date> {
        if i >= self.days.len() {
            return None;
        }
        if !self.validity[i] {
            return None;
        }
        Some(Date {
            days: self.days[i],
        })
    }

    /// # Safety
    /// No bounds checks.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _get(&self, i: usize) -> Option<Date> {
        if unsafe { *self.validity.get_unchecked(i) } {
            Some(Date {
                days: unsafe { *self.days.get_unchecked(i) },
            })
        } else {
            None
        }
    }

    /// # Safety
    /// No bounds checks on `idx`.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _take(&self, idx: &[usize]) -> Self {
        let m = idx.len();
        let mut out_days = Vec::with_capacity(m);
        unsafe { out_days.set_len(m); }

        let mut out_validity = bitvec![0; m];

        for (j, &i) in idx.iter().enumerate() {
            unsafe {
                *out_days.get_unchecked_mut(j) = *self.days.get_unchecked(i);
                out_validity.set(j, *self.validity.get_unchecked(i));
            }
        }

        Self {
            days: out_days,
            validity: out_validity,
        }
    }

    /// Safe indexed take.
    #[inline]
    pub fn take(&self, idx: &[usize]) -> ErrorResult<Self> {
        let n = self.days.len();
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

    /// # Safety
    /// No bounds checks on `mask`.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _bool_index(&self, mask: &[bool]) -> Self {
        let n = self.days.len();
        debug_assert_eq!(mask.len(), n);

        let mut out_days = Vec::<i32>::with_capacity(n);
        let mut out_validity = bitvec![0; n];

        unsafe {
            let mut j = 0usize;
            let out_ptr = out_days.as_mut_ptr();

            for i in 0..n {
                if *mask.get_unchecked(i) {
                    std::ptr::write(out_ptr.add(j), *self.days.get_unchecked(i));
                    out_validity.set_unchecked(j, *self.validity.get_unchecked(i));
                    j += 1;
                }
            }

            out_days.set_len(j);
            out_validity.truncate(j);
        }

        Self {
            days: out_days,
            validity: out_validity,
        }
    }

    /// Safe boolean indexing.
    #[inline]
    pub fn bool_index(&self, mask: &[bool]) -> ErrorResult<Self> {
        let n = self.days.len();
        if mask.len() != n {
            return Err(ErebusError::LengthMismatch {
                expected: n,
                found: mask.len(),
            });
        }
        Ok(self._bool_index(mask))
    }

    /// Returns an owned slice `[start, end)`.
    #[inline]
    pub fn slice(&self, start: usize, end: usize) -> Self {
        if start >= end || start >= self.days.len() {
            return Self {
                days: Vec::new(),
                validity: bitvec![0; 0],
            };
        }

        let end = end.min(self.days.len());
        Self {
            days: self.days[start..end].to_vec(),
            validity: self.validity[start..end].to_bitvec(),
        }
    }

//     /// Returns a zero-copy borrowed view.
//     #[inline]
//     pub fn slice_view(&self, start: usize, end: usize) -> DateVectorView<'_> {
//         let len = self.days.len();
//         if start >= end || start >= len {
//             return DateVectorView {
//                 days: &[],
//                 validity: BitSlice::empty(),
//             };
//         }
//
//         let end = end.min(len);
//         DateVectorView {
//             days: &self.days[start..end],
//             validity: &self.validity[start..end],
//         }
//     }
}