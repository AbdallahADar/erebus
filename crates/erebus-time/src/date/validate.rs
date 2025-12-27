// === Imports ===

use crate::prelude::*;
use crate::calendar::gregorian::days_in_month;

// === Impls ===

/// Strict Y/M/D validation.
/// Errors on invalid input.
#[inline]
pub(crate) fn validate_ymd_strict(
    year: i32,
    month: u8,
    day: u8,
) -> ErrorResult<()> {

    // Month: 1..=12
    if month < 1 || month > 12 {
        return Err(ErebusError::InvalidMonth(month));
    }

    let max_day = days_in_month(year, month);

    // Day: 1..=days_in_month(year, month)
    if day < 1 || day > max_day {
        return Err(ErebusError::InvalidDay {
            year,
            month,
            day,
        });
    }

    Ok(())
}

/// Lenient Y/M/D validation.
/// Returns `false` instead of error on invalid input.
#[inline]
pub(crate) fn validate_ymd_lenient(
    year: i32,
    month: u8,
    day: u8,
) -> bool {

    if month < 1 || month > 12 {
        return false;
    }

    let max_day = days_in_month(year, month);

    if day < 1 || day > max_day {
        return false;
    }

    true
}