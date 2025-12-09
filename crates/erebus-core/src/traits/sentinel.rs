// === Impl ===

/// Defines a deterministic sentinel value for invalid or missing data.
pub trait Sentinel {
    fn sentinel() -> Self;
    fn is_sentinel(&self) -> bool;
}

impl Sentinel for i64 {
    #[inline] fn sentinel() -> Self { i64::MIN }
    #[inline] fn is_sentinel(&self) -> bool { *self == i64::MIN }
}

impl Sentinel for f64 {
    #[inline] fn sentinel() -> Self { f64::MIN }
    #[inline] fn is_sentinel(&self) -> bool { *self == f64::MIN }
}

impl Sentinel for bool {
    #[inline] fn sentinel() -> Self { false }
    #[inline] fn is_sentinel(&self) -> bool { false } // bools have no invalid sentinel
}

impl Sentinel for String {
    #[inline] fn sentinel() -> Self { String::new() }
    #[inline] fn is_sentinel(&self) -> bool { self.is_empty() }
}