// === Imports ===
use crate::prelude::*;

// === Types ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekConvention {
    /// ISO-8601:
    /// - Monday = 1
    /// - Week 1 = first week with a Thursday
    ISO,

    /// US convention:
    /// - Sunday = 1
    /// - Week 1 starts Jan 1
    US,

    /// Explicit custom definition
    Custom {
        first_day: u8, // 1..=7
    },
}

// === Impls ===

impl WeekConvention {
    #[inline]
    pub fn map_weekday(&self, iso: u8) -> u8 {
        debug_assert!((1..=7).contains(&iso));

        match *self {
            WeekConvention::ISO => iso,
            WeekConvention::US => {
                // ISO: Mon=1..Sun=7
                // US:  Sun=1..Sat=7
                if iso == 7 { 1 } else { iso + 1 }
            }
            WeekConvention::Custom { first_day } => {
                // Rotate so `first_day` becomes 1
                let d = iso as i32 - first_day as i32;
                ((d + 7).rem_euclid(7) + 1) as u8
            }
        }
    }
}