// === Imports ===
use crate::prelude::*;
use std::io::Read;

// === Types ===

#[derive(Debug, Clone)]
pub struct HolidaySet {
    days: Vec<i32>, // days since epoch, sorted
}

// === Impls ===

impl HolidaySet {

    pub fn new(mut days: Vec<i32>) -> Self {
        days.sort_unstable();
        days.dedup();
        Self { days }
    }

    /// Load holidays from a CSV reader.
    /// Expected format:
    /// ```csv
    /// date
    /// 1977-01-17
    /// 1977-02-21
    /// ```
    pub fn from_csv_reader<R: Read>(mut reader: R) -> ErrorResult<Self> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let mut days = Vec::new();
        for (i, line) in buf.lines().enumerate() {
            // Skip header
            if i == 0 {
                continue;
            }
            let s = line.trim();
            if s.is_empty() {
                continue;
            }
            let date = Date::from_ymd_str(s, "%Y-%m-%d")?;
            days.push(date.days());
        }
        Ok(Self::new(days))
    }

    #[inline]
    pub fn is_holiday(&self, date: Date) -> bool {
        self.days.binary_search(&date.days()).is_ok()
    }
}