// === Imports ===
use crate::prelude::*;

// === Impl ===

#[inline]
pub fn validate_cut_bins<T: PartialOrd>(
    bins: &[T],
) -> ErrorResult<()> {
    if bins.is_empty() {
        return Err(ErebusError::InvalidCutBins {
            reason: "bins cannot be empty".into(),
        });
    }

    for w in bins.windows(2) {
        if w[0] >= w[1] {
            return Err(ErebusError::InvalidCutBins {
                reason: "bins must be strictly increasing".into(),
            });
        }
    }

    Ok(())
}

#[inline]
pub fn validate_cut_inputs(
    bins: &[f64],
    labels: &[String],
    bounded: bool,
) -> ErrorResult<()> {
    if bounded {
        if bins.len() < 2 {
            return Err(ErebusError::InvalidCutBins {
                reason: "bounded cut requires at least 2 bin edges".to_string(),
            });
        }
        let expected = bins.len() - 1;
        if labels.len() != expected {
            return Err(ErebusError::InvalidCutLabels {
                expected,
                found: labels.len(),
            });
        }
    } else {
        if bins.is_empty() {
            return Err(ErebusError::InvalidCutBins {
                reason: "unbounded cut requires at least 1 bin edge".to_string(),
            });
        }
        let expected = bins.len() + 1;
        if labels.len() != expected {
            return Err(ErebusError::InvalidCutLabels {
                expected,
                found: labels.len(),
            });
        }
    }

    // monotonicity: strictly increasing
    for w in bins.windows(2) {
        if !(w[0] < w[1]) {
            return Err(ErebusError::InvalidCutBins {
                reason: "bin edges must be strictly increasing".to_string(),
            });
        }
    }

    Ok(())
}

#[inline]
pub fn cut_value_old(
    x: f64,
    bins: &[f64],
    right: bool,
    bounded: bool,
) -> (i64, bool) {
    let idx = if right {
        bins.partition_point(|b| x > *b)
    } else {
        bins.partition_point(|b| x >= *b)
    };

    if bounded {
        // Outside bounds → NA
        if idx == 0 || idx == bins.len() {
            (-1, false)
        } else {
            ((idx - 1) as i64, true)
        }
    } else {
        // Implicit (-inf, inf)
        (idx as i64, true)
    }
}

#[inline]
pub fn cut_value<T, R, F>(
    x: T,
    bins: &[T],
    right: bool,
    bounded: bool,
    emit: F,
) -> (R, bool)
where
    T: PartialOrd,
    F: Fn(i64) -> R,
{
    let idx = if right {
        bins.partition_point(|b| x > *b)
    } else {
        bins.partition_point(|b| x >= *b)
    };

    if bounded {
        // Outside bounds → NA
        if idx == 0 || idx == bins.len() {
            (emit(0), false)
        } else {
            (emit((idx - 1) as i64), true)
        }
    } else {
        // Implicit (-inf, inf)
        (emit(idx as i64), true)
    }
}