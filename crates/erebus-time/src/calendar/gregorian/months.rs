// === Imports ===
use crate::prelude::*;

// === Impls ===

pub const MONTHS_SHORT: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Long English month names (strftime `%B`)
pub const MONTHS_LONG: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];

#[inline]
pub fn month_from_short(s: &str) -> Option<u8> {
    // We expect at least 3 chars; caller should ensure this.
    for (i, &name) in MONTHS_SHORT.iter().enumerate() {
        if s.starts_with(name) {
            return Some((i + 1) as u8);
        }
    }
    None
}

#[inline]
pub fn month_from_long(s: &str) -> Option<u8> {
    for (i, &name) in MONTHS_LONG.iter().enumerate() {
        if s.starts_with(name) {
            return Some((i + 1) as u8);
        }
    }
    None
}

#[inline]
pub fn short_month_name(month: u8) -> &'static str {
    unsafe { *MONTHS_SHORT.get_unchecked((month - 1) as usize) }
}

#[inline]
pub fn long_month_name(month: u8) -> &'static str {
    unsafe { *MONTHS_LONG.get_unchecked((month - 1) as usize) }
}