// === Imports ===
use crate::prelude::*;

// === Impls ===

/// Returns true if `year` is a leap year in the Gregorian calendar
/// Rules:
/// - Divisible by 4 = leap
/// - Except divisible by 100 = not leap
/// - Except divisible by 400 = leap
#[inline]
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

