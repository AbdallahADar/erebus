// === Imports ===
use crate::prelude::*;

// === Impls ===

/// Returns ISO weekday for a given `days since epoch`.
/// Convention:
/// - 1 = Monday
/// - 2 = Tuesday
/// - 3 = Wednesday
/// - 4 = Thursday
/// - 5 = Friday
/// - 6 = Saturday
/// - 7 = Sunday
#[inline]
pub fn days_to_weekday_iso(days: i32) -> u8 {
    // We want a stable mapping even for negative days.
    // Use Euclidean modulo semantics.
    // Let days=0 be Thursday (4).
    // Then weekday = ((days + 3) mod 7) + 1
    let d = (days as i64 + 3).rem_euclid(7) as u8;
    d + 1
}