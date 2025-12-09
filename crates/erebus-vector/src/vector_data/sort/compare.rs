// === Imports ===
use crate::prelude::*;

// === Impl ===

pub struct Compare<'a, T> {
    data: &'a [T],
    validity: &'a BitSlice,
    ascending: bool,
    nulls_last: bool,
}

impl<'a, T: PartialOrd> Compare<'a, T> {

    #[inline]
    pub fn new(
        data: &'a [T],
        validity: &'a BitSlice,
        ascending: bool,
        nulls_last: bool,
    ) -> Self {
        Self { data, validity, ascending, nulls_last }
    }

    #[inline]
    pub fn cmp(&self, i: usize, j: usize) -> Ordering {

        let a_valid = unsafe { *self.validity.get_unchecked(i) };
        let b_valid = unsafe { *self.validity.get_unchecked(j) };

        match (a_valid, b_valid) {
            (false, false) => i.cmp(&j),  // preserve input order of nulls
            (false, true) => if self.nulls_last { Ordering::Greater } else { Ordering::Less },
            (true, false) => if self.nulls_last { Ordering::Less } else { Ordering::Greater },
            (true, true) => {
                // Why partial_cmp instead of cmp? B/c of f64 which doesnt have Ord due to f64::NAN
                // So even though our validity check ensures no f64::NAN, we cannot implement an Ord
                // We could have implemented a custom SafeF64 or OrderedFloat but native float is better for us
                // So partial_cmp works for us which is essentially the same but return Option<Ordering>
                let ord = self.data[i].partial_cmp(&self.data[j]).unwrap_or(Ordering::Equal);
                if self.ascending { ord } else { ord.reverse() }
            }
        }
    }

}