// === Imports ===
use crate::prelude::*;
use std::ops::{Neg,Add, Sub, Mul, Div};

// === Impl ===

impl Neg for VectorData<i64> {
    type Output = VectorData<i64>;

    #[inline]
    fn neg(self) -> Self::Output {
        (&self).neg()   // calls unary op
    }
}

impl<'a> Neg for &'a VectorData<i64> {
    type Output = VectorData<i64>;

    #[inline]
    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl Add<f64> for VectorData<i64> {
    type Output = VectorData<f64>;
    fn add(self, rhs: f64) -> Self::Output {
        self.add_scalar(rhs)
    }
}

impl<'a> Add<f64> for &'a VectorData<i64> {
    type Output = VectorData<f64>;
    fn add(self, rhs: f64) -> Self::Output {
        self.add_scalar(rhs)
    }
}

impl Sub<f64> for VectorData<i64> {
    type Output = VectorData<f64>;
    fn sub(self, rhs: f64) -> Self::Output {
        self.sub_scalar(rhs)
    }
}

impl<'a> Sub<f64> for &'a VectorData<i64> {
    type Output = VectorData<f64>;
    fn sub(self, rhs: f64) -> Self::Output {
        self.sub_scalar(rhs)
    }
}

impl Mul<f64> for VectorData<i64> {
    type Output = VectorData<f64>;
    fn mul(self, rhs: f64) -> Self::Output {
        self.mul_scalar(rhs)
    }
}

impl<'a> Mul<f64> for &'a VectorData<i64> {
    type Output = VectorData<f64>;
    fn mul(self, rhs: f64) -> Self::Output {
        self.mul_scalar(rhs)
    }
}

impl Div<f64> for VectorData<i64> {
    type Output = VectorData<f64>;
    fn div(self, rhs: f64) -> Self::Output {
        self.div_scalar(rhs)
    }
}

impl<'a> Div<f64> for &'a VectorData<i64> {
    type Output = VectorData<f64>;
    fn div(self, rhs: f64) -> Self::Output {
        self.div_scalar(rhs)
    }
}

// Equivalent to inv div
impl Div<VectorData<i64>> for f64 {
    type Output = VectorData<f64>;
    fn div(self, rhs: VectorData<i64>) -> Self::Output {
        rhs.inv_div_scalar(self)
    }
}

impl<'a> Div<&'a VectorData<i64>> for f64 {
    type Output = VectorData<f64>;
    fn div(self, rhs: &'a VectorData<i64>) -> Self::Output {
        rhs.inv_div_scalar(self)
    }
}

impl Neg for VectorData<f64> {
    type Output = VectorData<f64>;

    #[inline]
    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl<'a> Neg for &'a VectorData<f64> {
    type Output = VectorData<f64>;

    #[inline]
    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl Add<f64> for VectorData<f64> {
    type Output = VectorData<f64>;
    fn add(self, rhs: f64) -> Self::Output {
        self.add_scalar(rhs)
    }
}

impl<'a> Add<f64> for &'a VectorData<f64> {
    type Output = VectorData<f64>;
    fn add(self, rhs: f64) -> Self::Output {
        self.add_scalar(rhs)
    }
}

impl Sub<f64> for VectorData<f64> {
    type Output = VectorData<f64>;
    fn sub(self, rhs: f64) -> Self::Output {
        self.sub_scalar(rhs)
    }
}

impl<'a> Sub<f64> for &'a VectorData<f64> {
    type Output = VectorData<f64>;
    fn sub(self, rhs: f64) -> Self::Output {
        self.sub_scalar(rhs)
    }
}

impl Mul<f64> for VectorData<f64> {
    type Output = VectorData<f64>;
    fn mul(self, rhs: f64) -> Self::Output {
        self.mul_scalar(rhs)
    }
}

impl<'a> Mul<f64> for &'a VectorData<f64> {
    type Output = VectorData<f64>;
    fn mul(self, rhs: f64) -> Self::Output {
        self.mul_scalar(rhs)
    }
}

impl Div<f64> for VectorData<f64> {
    type Output = VectorData<f64>;
    fn div(self, rhs: f64) -> Self::Output {
        self.div_scalar(rhs)
    }
}

impl<'a> Div<f64> for &'a VectorData<f64> {
    type Output = VectorData<f64>;
    fn div(self, rhs: f64) -> Self::Output {
        self.div_scalar(rhs)
    }
}

// Equivalent to inv div
impl Div<VectorData<f64>> for f64 {
    type Output = VectorData<f64>;
    fn div(self, rhs: VectorData<f64>) -> Self::Output {
        rhs.inv_div_scalar(self)
    }
}

impl<'a> Div<&'a VectorData<f64>> for f64 {
    type Output = VectorData<f64>;
    fn div(self, rhs: &'a VectorData<f64>) -> Self::Output {
        rhs.inv_div_scalar(self)
    }
}