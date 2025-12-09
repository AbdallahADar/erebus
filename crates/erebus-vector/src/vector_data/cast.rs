// === Imports ===
use crate::prelude::*;
use std::any::TypeId;

// === Impl ===

impl<T: 'static> VectorData<T> {
    #[inline]
    pub fn is_numeric(&self) -> bool {
        TypeId::of::<T>() == TypeId::of::<i64>()
            || TypeId::of::<T>() == TypeId::of::<f64>()
    }
}

impl VectorData<i64> {
    /// i64 → f64 (lossless widening)
    #[inline]
    pub fn to_float(&self) -> VectorData<f64> {
        self.map_unary_owned(|&x| x as f64)
    }

    /// i64 → bool (nonzero → true)
    #[inline]
    pub fn to_bool(&self) -> VectorData<bool> {
        self.map_unary_owned(|&x| x != 0)
    }

    /// i64 → String
    #[inline]
    pub fn to_text(&self) -> VectorData<String> {
        self.map_unary_owned(|x| x.to_string())
    }
}

impl VectorData<f64> {
    /// f64 → i64 (truncating)
    #[inline]
    pub fn to_int(&self) -> VectorData<i64> {
        self.map_unary_owned(|&x| x as i64)
    }

    /// f64 → bool (nonzero → true)
    #[inline]
    pub fn to_bool(&self) -> VectorData<bool> {
        self.map_unary_owned(|&x| x != 0.0)
    }

    /// f64 → String
    #[inline]
    pub fn to_text(&self) -> VectorData<String> {
        self.map_unary_owned(|x| x.to_string())
    }
}

impl VectorData<bool> {
    /// bool → i64 (false = 0, true = 1)
    #[inline]
    pub fn to_int(&self) -> VectorData<i64> {
        self.map_unary_owned(|&x| if x { 1 } else { 0 })
    }

    /// bool → f64 (false = 0.0, true = 1.0)
    #[inline]
    pub fn to_float(&self) -> VectorData<f64> {
        self.map_unary_owned(|&x| if x { 1.0 } else { 0.0 })
    }

    /// bool → String ("true" / "false")
    #[inline]
    pub fn to_text(&self) -> VectorData<String> {
        self.map_unary_owned(|&x| x.to_string())
    }
}

impl VectorData<String> {
    /// String → i64 (parse; invalid → null)
    #[inline]
    pub fn to_int(&self) -> VectorData<i64> {
        self.map_unary_owned_with_validity(|s| match s.parse::<i64>() {
            Ok(v) => (v, true),
            Err(_) => (0, false),
        })
    }

    /// String → f64 (parse; invalid → null)
    #[inline]
    pub fn to_float(&self) -> VectorData<f64> {
        self.map_unary_owned_with_validity(|s| match s.parse::<f64>() {
            Ok(v) => (v, true),
            Err(_) => (f64::NAN, false),
        })
    }

    /// String → bool ("true" → true; others false)
    #[inline]
    pub fn to_bool(&self) -> VectorData<bool> {
        self.map_unary_owned_with_validity(|s| {
            if s.eq_ignore_ascii_case("true") {
                (true, true)
            } else if s.eq_ignore_ascii_case("false") {
                (false, true)
            } else {
                (false, false)
            }
        })
    }
}