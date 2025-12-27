// === Imports ===
use crate::prelude::*;

// === Impls ===

impl Date {
    #[inline]
    pub fn weekday(&self) -> u8 {
        days_to_weekday_iso(self.days)
    }

    #[inline]
    pub fn weekday_with(&self, conv: WeekConvention) -> u8 {
        conv.map_weekday(self.weekday())
    }
}

impl DateVector {

    #[inline]
    pub fn weekdays(&self) -> Vec<u8> {
        let n = self.days.len();
        let mut out = vec![0u8; n];

        for i in 0..n {
            if unsafe { *self.validity.get_unchecked(i) } {
                out[i] = days_to_weekday_iso(self.days[i]);
            }
        }
        out
    }

    #[inline]
    pub fn weekdays_with(&self, conv: WeekConvention) -> Vec<u8> {
        let n = self.days.len();
        let mut out = vec![0u8; n];

        for i in 0..n {
            unsafe {
                if *self.validity.get_unchecked(i) {
                    let iso = days_to_weekday_iso(*self.days.get_unchecked(i));
                    out[i] = conv.map_weekday(iso);
                }
            }
        }
        out
    }

    pub fn count_weekdays(&self, rule: WeekendRule) -> usize {
        let mut count = 0;
        let n = self.days.len();

        for i in 0..n {
            if unsafe { *self.validity.get_unchecked(i) } {
                if !rule.is_weekend(Date { days: self.days[i] }) {
                    count += 1;
                }
            }
        }
        count
    }

}