// === Imports ===
use crate::prelude::*;

// === Impl ===

impl<T> VectorData<T> {

    /// # Safety
    /// Performs no bounds checks. Intended for internal fast-path usage.
    /// Enabled only under the `internal` feature.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _is_valid_at(&self, i: usize) -> bool {
        debug_assert!(i < self.validity.len(), "index out of bounds");
        unsafe { *self.validity.get_unchecked(i) }
    }

    /// Returns whether the value at index `i` is valid (non-null).
    /// Returns an error if `i` is out of bounds.
    #[inline]
    pub fn is_valid_at(&self, i: usize) -> ErrorResult<bool> {
        if i >= self.validity.len() {
            return Err(ErebusError::IndexOutOfBounds {
                index: i,
                size: self.validity.len(),
            });
        }
        Ok(self._is_valid_at(i))
    }

    /// # Safety
    /// Replaces validity bitmap without verifying length.
    /// Intended for internal or performance-critical paths.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _set_validity(&mut self, validity: BitVec) {
        debug_assert_eq!(self.data.len(), validity.len(), "length mismatch");
        self.validity = validity;
    }

    /// Safe wrapper for setting validity bitmap in place.
    /// Returns an error if lengths mismatch.
    #[inline]
    pub fn set_validity(&mut self, validity: BitVec) -> ErrorResult<()> {
        if self.data.len() != validity.len() {
            return Err(ErebusError::LengthMismatch {
                expected: self.data.len(),
                found: validity.len(),
            });
        }
        self._set_validity(validity);
        Ok(())
    }

    /// # Safety
    /// Consumes and replaces validity bitmap without verifying length.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _with_validity(mut self, validity: BitVec) -> Self {
        debug_assert_eq!(self.data.len(), validity.len(), "length mismatch");
        self.validity = validity;
        self
    }

    /// Functional style: returns new [`VectorData`] with updated validity.
    #[inline]
    pub fn with_validity(mut self, validity: BitVec) -> ErrorResult<Self> {
        if self.data.len() != validity.len() {
            return Err(ErebusError::LengthMismatch {
                expected: self.data.len(),
                found: validity.len(),
            });
        }
        Ok(self._with_validity(validity))
    }
}