// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<i64> {

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
            cut_value(*x as f64, bins, right, bounded, |idx| idx)
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
            let xf = *x as f64;
            cut_value(xf, bins, right, bounded, |idx| labels[idx as usize].clone())
        }))
    }

}