// === Imports ===

use crate::prelude::*;
use super::convert::*;
use super::validate::*;
use super::parse::{compile_ymd_format, parse_ymd_strict,
    YmdExtractor, parse_ymd_lenient};

// === Types ===

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub(crate) days: i32,
}

#[derive(Debug, Clone)]
pub struct DateVector {
    pub(crate) days: Vec<i32>,
    pub(crate) validity: BitVec,
}

// === Impls ===

impl Date {

    #[inline]
    pub fn from_ymd(year: i32, month: u8, day: u8) -> ErrorResult<Self> {
        validate_ymd_strict(year, month, day)?;
        let days = ymd_to_days(year, month, day);
        Ok(Self { days })
    }

    pub fn from_ymd_str(s: &str, fmt: &str) -> ErrorResult<Self> {
        let extractor = compile_ymd_format(fmt)?;
        let days = parse_ymd_strict(s, &extractor)?;
        Ok(Self { days })
    }

    #[inline]
    pub fn days(&self) -> i32 {
        self.days
    }
}

impl DateVector {

    #[inline]
    pub fn len(&self) -> usize {
        self.days.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.days.is_empty()
    }

    /// - No bounds checks are performed
    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) unsafe fn _from_ymd_vectors(
        years: &[i32],
        months: &[u8],
        days_in: &[u8],
    ) -> Self {
        let n = years.len();

        let mut days = vec![0i32; n];
        let mut validity = bitvec![0; n];

        for i in 0..n {
            let y = *years.get_unchecked(i);
            let m = *months.get_unchecked(i);
            let d = *days_in.get_unchecked(i);

            if validate_ymd_lenient(y, m, d) {
                *days.get_unchecked_mut(i) = ymd_to_days(y, m, d);
                validity.set_unchecked(i, true);
            }
        }

        Self { days, validity }
    }

    /// Construct from Y/M/D vectors (lenient per-element validation).
    /// Invalid dates are encoded as nulls.
    #[inline]
    pub fn from_ymd_vectors(
        years: &[i32],
        months: &[u8],
        days: &[u8],
    ) -> ErrorResult<Self> {
        let n = years.len();

        if months.len() != n || days.len() != n {
            return Err(ErebusError::LengthMismatch {
                expected: n,
                found: months.len().min(days.len()),
            });
        }

        // SAFETY: lengths verified above
        Ok(unsafe {
            Self::_from_ymd_vectors(years, months, days)
        })
    }

    #[cfg_attr(feature = "internal", visibility::make(pub))]
    #[inline]
    pub(crate) unsafe fn _from_ymd_str_vectors(
        values: &[&str],
        extractor: &YmdExtractor,
    ) -> Self {
        let n = values.len();

        let mut days = vec![0i32; n];
        let mut validity = bitvec![0; n];

        for i in 0..n {
            let s = *values.get_unchecked(i);

            if let Some(v) = parse_ymd_lenient(s, extractor) {
                *days.get_unchecked_mut(i) = v;
                validity.set_unchecked(i, true);
            }
        }

        Self { days, validity }
    }

    #[inline]
    pub fn from_ymd_str_vectors(
        values: &[&str],
        fmt: &str,
    ) -> ErrorResult<Self> {

        let extractor = compile_ymd_format(fmt)?;

        Ok(unsafe {
            Self::_from_ymd_str_vectors(values, &extractor)
        })
    }

}