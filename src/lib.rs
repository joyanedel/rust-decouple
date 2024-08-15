pub mod core;
mod traits;

#[cfg(feature = "derive")]
extern crate rust_decouple_derive;
#[cfg(feature = "derive")]
pub use rust_decouple_derive::Decouple;
#[cfg(feature = "derive")]
pub use traits::Decouple;
