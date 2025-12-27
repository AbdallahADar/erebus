// === Imports ===
use crate::prelude::*;

// === Impls ===

/// Returns the number of days in the given month of the given year.
/// - No error handling here by design
#[inline]
pub fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 => 31, // Jan
        2 => if is_leap_year(year) { 29 } else { 28 }, // Feb
        3 => 31, // Mar
        4 => 30, // Apr
        5 => 31, // May
        6 => 30, // Jun
        7 => 31, // Jul
        8 => 31, // Aug
        9 => 30, // Sep
        10 => 31, // Oct
        11 => 30, // Nov
        12 => 31, // Dec
        _ => 0,  // unreachable if validated upstream
    }
}