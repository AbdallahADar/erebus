// === Impl ===

pub trait Numeric:
    Copy + Default + Send + Sync +
    PartialEq +
    std::ops::AddAssign<Self> +
    std::ops::MulAssign<Self> +
    'static
{
    fn zero() -> Self;
    fn one() -> Self;
    fn to_f64(self) -> f64;      // for mixed-type aggregates
    fn abs(self) -> Self;        // for norms
}
impl Numeric for i64 {
    #[inline] fn zero() -> Self { 0 }
    #[inline] fn one() -> Self { 1 }
    #[inline] fn to_f64(self) -> f64 { self as f64 }
    #[inline] fn abs(self) -> Self { self.abs() }
}
impl Numeric for f64 {
    #[inline] fn zero() -> Self { 0.0 }
    #[inline] fn one() -> Self { 1.0 }
    #[inline] fn to_f64(self) -> f64 { self }
    #[inline] fn abs(self) -> Self { self.abs() }
}
