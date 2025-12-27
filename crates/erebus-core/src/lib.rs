// === erebus-core ===
// Shared traits, errors, prelude

pub mod errors;
pub mod traits;
pub mod runtime;
pub mod prelude;
pub mod algorithms;
pub mod result;

pub use errors::ErebusError;
pub use traits::*;
pub use runtime::*;
pub use algorithms::*;
pub use result::*;