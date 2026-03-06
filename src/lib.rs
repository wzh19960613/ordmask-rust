#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

mod macros;
mod ops;

mod error;
mod ordmask;
mod with_min;

pub use error::*;
pub use ordmask::*;
pub use with_min::*;

/// Utility functions
pub mod utils;
