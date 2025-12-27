// === Imports ===
use crate::prelude::*;

// === Impl ===

impl DateVector {

    /// Performs no bounds checks.
    /// Intended for internal fast-path usage.
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

    /// Replaces validity bitmap without verifying length.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _set_validity(&mut self, validity: BitVec) {
        debug_assert_eq!(self.days.len(), validity.len(), "length mismatch");
        self.validity = validity;
    }

    /// Safe wrapper for setting validity bitmap.
    #[inline]
    pub fn set_validity(&mut self, validity: BitVec) -> ErrorResult<()> {
        if self.days.len() != validity.len() {
            return Err(ErebusError::LengthMismatch {
                expected: self.days.len(),
                found: validity.len(),
            });
        }
        self._set_validity(validity);
        Ok(())
    }

    /// Consumes self and replaces validity bitmap without verifying length.
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) fn _with_validity(mut self, validity: BitVec) -> Self {
        debug_assert_eq!(self.days.len(), validity.len(), "length mismatch");
        self.validity = validity;
        self
    }

    /// Safe wrapper for replacing validity bitmap.
    #[inline]
    pub fn with_validity(self, validity: BitVec) -> ErrorResult<Self> {
        if self.days.len() != validity.len() {
            return Err(ErebusError::LengthMismatch {
                expected: self.days.len(),
                found: validity.len(),
            });
        }
        Ok(self._with_validity(validity))
    }
}