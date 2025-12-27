// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<f64> {

    // -- Cut --
    #[inline]
    pub fn cut(
        &self,
        bins: &[f64],
        right: bool,
        bounded: bool,
    ) -> Result<VectorData<i64>, ErebusError> {
        validate_cut_bins(bins)?;
        Ok(self.map_unary_owned_with_validity(|x| {
            cut_value(*x, bins, right, bounded, |idx| idx)
        }))
    }

    // -- Cut with Labels --
    #[inline]
    pub fn cut_labels(
        &self,
        bins: &[f64],
        labels: &[String],
        right: bool,
        bounded: bool,
    ) -> Result<VectorData<String>, ErebusError> {
        validate_cut_inputs(bins, labels, bounded)?;
        Ok(self.map_unary_owned_with_validity(|x| {
            cut_value(*x, bins, right, bounded, |idx| {
                // idx is guaranteed in-range if inputs are validated;
                // invalid rows return (emit(0), false) and validity masks it.
                labels[idx as usize].clone()
            })
        }))
    }

}