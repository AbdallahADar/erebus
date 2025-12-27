// === Imports ===
use crate::prelude::*;

// === Types ===

pub struct MarketCalendar {
    weekend: WeekendRule,
    holidays: HolidaySet,
}

// === Impls ===

impl MarketCalendar {
    #[inline]
    pub fn new(
        weekend: WeekendRule,
        holidays: HolidaySet,
    ) -> Self {
        Self { weekend, holidays }
    }
}

impl Calendar for MarketCalendar {
    #[inline]
    fn is_weekend(&self, date: Date) -> bool {
        self.weekend.is_weekend(date)
    }

    #[inline]
    fn is_holiday(&self, date: Date) -> bool {
        self.holidays.is_holiday(date)
    }
}