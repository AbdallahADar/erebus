// === Imports ===

use crate::prelude::*;

// === Impls ===

// This march-based years is taking inspiration from Howard Hinnant’s algorithms
// https://howardhinnant.github.io/date_algorithms.html

/// Number of days from 0000-03-01 to 1970-01-01.
/// Shift the epoch to March-based years to simplify leap-year handling.
/// This constant anchors the internal day count to Unix epoch.
const UNIX_EPOCH_DAYS: i32 = 719468;

/// Converts a valid (year, month, day) to days since Unix epoch (1970-01-01).
#[inline]
pub fn ymd_to_days(year: i32, month: u8, day: u8) -> i32 {

    // Convert month to March-based indexing:
    // March = 0, April = 1, ..., January = 10, February = 11
    let (y, m) = if month <= 2 {
        (year - 1, month as i32 + 9)
    } else {
        (year, month as i32 - 3)
    };

    let era = y.div_euclid(400);
    let yoe = y - era * 400; // [0, 399]
    let doy = (153 * m + 2) / 5 + day as i32 - 1; // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;

    era * 146097 + doe - UNIX_EPOCH_DAYS
}

#[inline]
pub fn days_to_ymd(days: i32) -> (i32, u8, u8) {
    // Shift back to civil-from-days epoch (0000-03-01 based)
    let z = days + 719468;

    let era = z.div_euclid(146097);
    let doe = z - era * 146097;                  // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i32 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;               // [0, 11]

    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };

    (year, m as u8, d as u8)
}