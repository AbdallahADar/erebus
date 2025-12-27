// === Imports ===
use crate::prelude::*;

// === Types ===

pub trait Calendar {
    fn is_weekend(&self, date: Date) -> bool;
    fn is_holiday(&self, date: Date) -> bool;

    #[inline]
    fn is_business_day(&self, date: Date) -> bool {
        !self.is_weekend(date) && !self.is_holiday(date)
    }
}