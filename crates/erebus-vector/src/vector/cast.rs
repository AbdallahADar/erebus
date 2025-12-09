// === Imports ===
use crate::prelude::*;
use std::any::TypeId;

// === Impl ===

impl<T: 'static> Vector<T> {
    #[inline]
    pub fn is_numeric(&self) -> bool {
        TypeId::of::<T>() == TypeId::of::<i64>()
            || TypeId::of::<T>() == TypeId::of::<f64>()
    }
}

impl Vector<i64> {
    pub fn to_float(&self) -> Vector<f64> {
        self.map_unary_owned(|&x| x as f64)
    }

    pub fn to_bool(&self) -> Vector<bool> {
        self.map_unary_owned(|&x| x != 0)
    }

    pub fn to_text(&self) -> Vector<String> {
        self.map_unary_owned(|x| x.to_string())
    }
}

impl Vector<f64> {
    pub fn to_int(&self) -> Vector<i64> {
        self.map_unary_owned(|&x| x as i64)
    }

    pub fn to_bool(&self) -> Vector<bool> {
        self.map_unary_owned(|&x| x != 0.0)
    }

    pub fn to_text(&self) -> Vector<String> {
        self.map_unary_owned(|x| x.to_string())
    }
}

impl Vector<bool> {
    pub fn to_int(&self) -> Vector<i64> {
        self.map_unary_owned(|&x| if x { 1 } else { 0 })
    }

    pub fn to_float(&self) -> Vector<f64> {
        self.map_unary_owned(|&x| if x { 1.0 } else { 0.0 })
    }

    pub fn to_text(&self) -> Vector<String> {
        self.map_unary_owned(|&x| x.to_string())
    }
}

impl Vector<String> {
    pub fn to_int(&self) -> Vector<i64> {
        self.map_unary_owned(|s| s.parse::<i64>().unwrap_or_else(|_| i64::sentinel()))
    }

    /// String → f64 (invalid or NaN → sentinel f64::MIN)
    pub fn to_float(&self) -> Vector<f64> {
        self.map_unary_owned(|s| {
            match s.trim().to_lowercase().as_str() {
                "nan" => f64::MIN, // explicitly treat literal "NaN" as invalid
                _ => s.parse::<f64>().unwrap_or(f64::MIN),
            }
        })
    }

    pub fn to_bool(&self) -> Vector<bool> {
        self.map_unary_owned(|s| s.eq_ignore_ascii_case("true"))
    }
}