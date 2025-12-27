// === Imports ===
use crate::prelude::*;

// === Types ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekendRule {

    /// Saturday + Sunday are weekend.
    SaturdaySunday,

    /// Friday + Saturday are weekend (common in some regions).
    FridaySaturday,

    /// Explicit set of weekend days.
    /// Indices follow ISO weekday ordering:
    /// 1=Mon, 2=Tue, 3=Wed, 4=Thu, 5=Fri, 6=Sat, 7=Sun
    Custom {
        fri: bool,
        sat: bool,
        sun: bool,
    },
}

// === Impls ===

impl Default for WeekendRule {
    #[inline]
    fn default() -> Self {
        WeekendRule::SaturdaySunday
    }
}

impl WeekendRule {

    /// Returns true if `date` falls on a weekend day under this rule.
    #[inline]
    pub fn is_weekend(&self, date: Date) -> bool {
        // ISO weekday: 1..=7 (Mon..Sun)
        let wd = days_to_weekday_iso(date.days());

        match *self {
            WeekendRule::SaturdaySunday => wd == 6 || wd == 7,
            WeekendRule::FridaySaturday => wd == 5 || wd == 6,
            WeekendRule::Custom { fri, sat, sun } => {
                (wd == 5 && fri) || (wd == 6 && sat) || (wd == 7 && sun)
            }
        }
    }
}