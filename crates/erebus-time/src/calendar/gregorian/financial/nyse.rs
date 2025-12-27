// === Imports ===
use crate::prelude::*;
use std::io::Cursor;

// === Impls ===

const NYSE_HOLIDAYS_CSV: &str =
    include_str!("../../../../data/calendars/nyse/nyse_holidays.csv");

/// Returns the NYSE market calendar.
pub fn get_nyse_calendar() -> ErrorResult<MarketCalendar> {
    let cursor = Cursor::new(NYSE_HOLIDAYS_CSV.as_bytes());
    let holidays = HolidaySet::from_csv_reader(cursor)?;

    Ok(MarketCalendar::new(
        WeekendRule::SaturdaySunday,
        holidays,
    ))
}