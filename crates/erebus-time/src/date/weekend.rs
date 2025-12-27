// === Imports ===
use crate::prelude::*;

// === Impls ===

impl Date {
    pub fn is_weekend(&self, rule: WeekendRule) -> bool {
        rule.is_weekend(*self)
    }

    pub fn is_business_day<C: Calendar>(&self, cal: &C) -> bool {
        cal.is_business_day(*self)
    }

    #[inline]
    pub fn shift_business_days<C: Calendar>(&self, cal: &C, n: i32) -> Date {
        if n == 0 {
            return *self;
        }
        let mut remaining = n.abs();
        let mut d = self.days;
        if n > 0 {
            while remaining > 0 {
                d += 1;
                if cal.is_business_day(Date { days: d }) {
                    remaining -= 1;
                }
            }
        } else {
            while remaining > 0 {
                d -= 1;
                if cal.is_business_day(Date { days: d }) {
                    remaining -= 1;
                }
            }
        }
        Date { days: d }
    }

    #[inline]
    pub fn next_business_day<C: Calendar>(&self, cal: &C) -> Date {
        self.shift_business_days(cal, 1)
    }

    #[inline]
    pub fn previous_business_day<C: Calendar>(&self, cal: &C) -> Date {
        self.shift_business_days(cal, -1)
    }
}

impl DateVector {
    pub fn is_weekend(&self, rule: WeekendRule) -> BitVec {
        let n = self.days.len();
        let mut out = bitvec![0; n];
        for i in 0..n {
            unsafe {
                if *self.validity.get_unchecked(i) {
                    let d = *self.days.get_unchecked(i);
                    if rule.is_weekend(Date { days: d }) {
                        out.set_unchecked(i, true);
                    }
                }
            }
        }
        out
    }

    pub fn is_business_day<C: Calendar>(&self, cal: &C) -> BitVec {
        let n = self.days.len();
        let mut out = bitvec![0; n];
        for i in 0..n {
            unsafe {
                if *self.validity.get_unchecked(i) {
                    let d = *self.days.get_unchecked(i);
                    if cal.is_business_day(Date { days: d }) {
                        out.set_unchecked(i, true);
                    }
                }
            }
        }
        out
    }

    pub fn shift_business_days<C: Calendar>(&self, cal: &C, n: i32) -> DateVector {
        let len = self.days.len();
        let mut out = vec![0i32; len];
        let mut validity = self.validity.clone();

        for i in 0..len {
            if unsafe { *self.validity.get_unchecked(i) } {
                let d = Date { days: self.days[i] };
                out[i] = d.shift_business_days(cal, n).days();
            }
        }

        DateVector { days: out, validity }
    }

    #[inline]
    pub fn next_business_day<C: Calendar>(&self, cal: &C) -> DateVector {
        self.shift_business_days(cal, 1)
    }

    #[inline]
    pub fn previous_business_day<C: Calendar>(&self, cal: &C) -> DateVector {
        self.shift_business_days(cal, -1)
    }

    pub fn count_business_days<C: Calendar>(&self, cal: &C) -> usize {
        let mut count = 0;
        let n = self.days.len();

        for i in 0..n {
            if unsafe { *self.validity.get_unchecked(i) } {
                if cal.is_business_day(Date { days: self.days[i] }) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn count_weekends(&self, rule: WeekendRule) -> usize {
        let mut count = 0;
        let n = self.days.len();

        for i in 0..n {
            if unsafe { *self.validity.get_unchecked(i) } {
                if rule.is_weekend(Date { days: self.days[i] }) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn count_holidays<C: Calendar>(&self, cal: &C) -> usize {
        let mut count = 0;
        let n = self.days.len();

        for i in 0..n {
            if unsafe { *self.validity.get_unchecked(i) } {
                if cal.is_holiday(Date { days: self.days[i] }) {
                    count += 1;
                }
            }
        }
        count
    }

}