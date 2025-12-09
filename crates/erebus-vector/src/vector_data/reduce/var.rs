// === Imports ===
use crate::prelude::*;

// === Impl ===

#[derive(Clone)]
pub struct VarAcc {
    pub count: usize,
    pub mean: f64,
    pub m2: f64,
}

impl Default for VarAcc {
    #[inline]
    fn default() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            m2: 0.0,
        }
    }
}

#[derive(Clone)]
pub struct VarianceReducer {
    pub ddof: usize, // 0 = population, 1 = sample
}

impl<T: Numeric> Reducer<T> for VarianceReducer {
    type Acc = VarAcc;
    type Output = f64;

    #[inline]
    fn accumulate(&mut self, acc: &mut Self::Acc, value: &T, is_valid: bool) {
        if !is_valid {
            return;
        }

        let x = (*value).to_f64();

        acc.count += 1;

        // Welford update
        let delta = x - acc.mean;
        acc.mean += delta / acc.count as f64;
        let delta2 = x - acc.mean;
        acc.m2 += delta * delta2;
    }

    #[inline]
    fn combine(&self, a: &mut Self::Acc, b: Self::Acc) {
        // If no contributions
        if b.count == 0 {
            return;
        }

        let total = a.count + b.count;
        let delta = b.mean - a.mean;

        // Update M2
        a.m2 += b.m2 + delta * delta * (a.count as f64) * (b.count as f64) / (total as f64);

        // Update mean
        a.mean = (a.mean * a.count as f64 + b.mean * b.count as f64) / total as f64;

        // Update count
        a.count = total;
    }

    #[inline]
    fn finalize(&self, acc: Self::Acc) -> Self::Output {
        if acc.count <= self.ddof {
            return f64::NAN; // undefined
        }
        acc.m2 / ((acc.count - self.ddof) as f64)
    }
}

impl<T: Numeric> VectorData<T> {
    /// Variance with optional `ddof`
    #[inline]
    pub fn var(&self, ddof: Option<usize>) -> f64 {
        self._reduce(VarianceReducer {
            ddof: ddof.unwrap_or(0),
        })
    }

    /// Standard deviation with optional ddof
    #[inline]
    pub fn std(&self, ddof: Option<usize>) -> f64 {
        self.var(ddof).sqrt()
    }
}