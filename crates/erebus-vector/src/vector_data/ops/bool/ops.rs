// === Imports ===
use crate::prelude::*;
use super::super::macros::*;

// === Impl ===

impl VectorData<bool> {

    // -- Not --
    impl_unary_op!(
        noparams, inplace,
        not, not_inplace, not_range,
        bool,
        |x: &bool| !*x,
        |x: &mut bool| *x = !*x
    );
}