//! Commonly used types and traits.
//!
//! This module re-exports the most commonly used items from this crate.
//! Importing this module with `use ordmask::prelude::*;` brings the essential
//! types, macros, and traits into scope.
//!
//! # Re-exports
//!
//! - [`OrdMask`] - The main mask type for efficient range-based membership checking
//! - [`ordmask`] - Macro for creating `OrdMask` values
//! - [`WithMin`] - Trait for types with a minimum value (required for `OrdMask`)
//! - [`WithMax`] - Trait for types with a maximum value (required for spans)
//! - [`WithOne`] - Trait for types with a "one" value (required for value iteration)
//! - [`WithZero`] - Trait for types with a "zero" value (required for count operations)
//! - [`OrderedSub`] - Trait for ordered subtraction (required for values count)
//! - [`OrdMaskError`] - Error type for this crate

pub use crate::{Error as OrdMaskError, OrdMask, ordmask};

pub use crate::{OrderedSub, WithMax, WithMin, WithOne, WithZero};
