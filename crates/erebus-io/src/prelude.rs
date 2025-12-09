/// Declare necessary imports in this common input file

// Standard types
pub use std::cmp::Ordering;

// === Erebus core ===
pub use erebus_core::prelude::*;

// Vector-level internal exports
pub use crate::format::*;
pub use crate::write::*;
pub use crate::read::*;
pub use crate::vector_data::*;
pub use crate::utils::*;
pub use crate::compression::*;