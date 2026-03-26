//! Module for efficiently iterating over the included spans and values of an [`OrdMask`](crate::OrdMask).

pub mod spans;
pub mod values;

mod values_count;
pub use values_count::*;
