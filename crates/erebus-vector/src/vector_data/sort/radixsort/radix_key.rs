// === Imports ===
use crate::prelude::*;
use std::ops::{BitAnd, Shr};

// === Impl ===

/// Trait for extracting a sortable radix key from a type.
/// Key must be an unsigned integer type (u64 here).
pub trait RadixKey {
    /// Key type (u64 for all numeric types)
    type Key: Copy
        + Default
        + Ord
        + BitAnd<Output = Self::Key>
        + Shr<usize, Output = Self::Key>;

    /// Convert a value to sortable bits
    fn to_radix_key(v: Self) -> Self::Key;

    /// Mask for one byte (always 0xFF)
    #[inline]
    fn ff_mask() -> Self::Key;

    /// Extract the byte at a given shift (0, 8, 16, …)
    fn extract_byte(k: Self::Key, shift: usize) -> usize;
}

impl RadixKey for i64 {
    type Key = u64;
    fn to_radix_key(v: i64) -> u64 { (v as u64) ^ 0x8000_0000_0000_0000 }
    fn ff_mask() -> u64 { 0xFF }
    fn extract_byte(k: u64, shift: usize) -> usize { ((k >> shift) & 0xFF) as usize }
}

impl RadixKey for f64 {
    type Key = u64;

    #[inline]
    fn to_radix_key(v: f64) -> u64 {
        // Create sortable IEEE-754 order-preserving key
        let bits = v.to_bits();
        if bits & 0x8000_0000_0000_0000 == 0 {
            // Positive numbers: flip highest bit
            bits ^ 0x8000_0000_0000_0000
        } else {
            // Negative numbers: flip all bits
            !bits
        }
    }

    #[inline]
    fn ff_mask() -> u64 { 0xFF }

    #[inline]
    fn extract_byte(k: u64, shift: usize) -> usize {
        ((k >> shift) & 0xFF) as usize
    }
}

// Simply defining these so that we can implement trait RadixKey on the sort functions and call it
// These will not be used as of yet

impl RadixKey for bool {
    type Key = u64;

    #[inline]
    fn to_radix_key(_v: bool) -> u64 { 0 }

    #[inline]
    fn ff_mask() -> u64 { 0xFF }

    #[inline]
    fn extract_byte(_k: u64, _shift: usize) -> usize { 0 }
}

impl RadixKey for String {
    type Key = u64;

    #[inline]
    fn to_radix_key(_v: String) -> u64 { 0 }

    #[inline]
    fn ff_mask() -> u64 { 0xFF }

    #[inline]
    fn extract_byte(_k: u64, _shift: usize) -> usize { 0 }
}