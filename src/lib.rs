#![deny(missing_docs)]

//! [> Chinese Version: 中文文档](https://github.com/wzh19960613/ordmask-rust/blob/master/README_CN.md)
//!
//! `ordmask` is a library for efficient range-based set operations and membership checking.
//! It represents a set of values as a collection of intervals and supports various set operations.
//!
//! # Features
//!
//! - Efficient range membership checking
//! - Support for [`union`](OrdMask::union), [`intersection`](OrdMask::intersection),
//!   [`minus`](OrdMask::minus), [`complement`](OrdMask::complement), and
//!   [`symmetric_difference`](OrdMask::symmetric_difference) operations
//! - Works with any type that implements [`Ord`], [`Clone`], and [`WithMin`] traits
//! - Zero-allocation operations where possible
//! - Optional `serde` feature for serialization/deserialization
//!
//! # Construction
//!
//! ```
//! use ordmask::{OrdMask, ordmask};
//!
//! // [0, 10), [20, 30) and [40, MAX]
//! let mask = ordmask![0, 10, 20, 30, 40];
//! assert!(mask.included(&5));
//! assert!(mask.excluded(&10));
//! assert!(mask.included(&50));
//!
//! // Create from `Vec<T>`
//! assert_eq!(mask, OrdMask::from(vec![0, 10, 20, 30, 40]));
//!
//! // Create from suspicious_points and a predicate
//! use std::collections::BTreeSet;
//! assert_eq!(mask, OrdMask::from_suspicious_points_set(
//!     BTreeSet::from([0, 10, 20, 30, 40]),
//!     |x| matches!(x, 0..10 | 20..30 | 40..),
//!     false
//! ));
//!
//! // Create from suspicious_points_map
//! use std::collections::BTreeMap;
//! let map = BTreeMap::from([(0, true), (10, false), (20, true), (30, false), (40, true)]);
//! assert_eq!(mask, OrdMask::from_suspicious_points_map(map, false));
//!
//! // [MIN, 10)
//! let mask = ordmask![.., 10];
//! assert_eq!(mask, OrdMask::less_than(10));
//! assert!(mask.included(&9));
//! assert!(mask.excluded(&10));
//!
//! // [10, MAX]
//! let mask = ordmask![10];
//! assert_eq!(mask, OrdMask::not_less_than(10));
//! assert!(mask.excluded(&9));
//! assert!(mask.included(&10));
//!
//! // [10, 20)
//! let mask = ordmask![10, 20];
//! assert_eq!(mask, OrdMask::in_range(10, 20));
//! assert!(mask.included(&10));
//! assert!(mask.included(&15));
//! assert!(mask.excluded(&20));
//! assert!(mask.excluded(&25));
//!
//! // Universal
//! let mask = ordmask![..];
//! assert_eq!(mask, OrdMask::universal());
//! assert!(mask.is_universal());
//! assert!(mask.included(&0));
//!
//! // Empty
//! let mask = ordmask![];
//! assert_eq!(mask, OrdMask::empty());
//! assert!(mask.is_empty());
//! assert!(mask.excluded(&0));
//! ```
//!
//! ## Type Annotation
//!
//! You can specify the type explicitly using the `<T>` syntax in the macro:
//!
//! ```
//! use ordmask::{OrdMask, ordmask};
//!
//! // Explicit type annotation with <T>
//! let mask = ordmask![<i64>];        // Empty
//! let mask = ordmask![<u8> ..];      // Universal
//! let mask = ordmask![<u64> 10];     // [10, MAX]
//! let mask = ordmask![<i32> 10, 20]; // [10, 20)
//! let mask = ordmask![<u32> .., 10]; // [MIN, 10)
//! ```
//!
//! # Union
//!
//! ```
//! use ordmask::{OrdMask, ordmask};
//!
//! let a = ordmask![0, 15];
//! let b = ordmask![5, 20];
//! let c = ordmask![10, 30];
//! // &a | &b | &c: reference operators do not move (consume) the values
//! assert_eq!(&a | &b | &c, OrdMask::union(&[&a, &b, &c]));
//! // a | b | c: non-reference operators move (consume) the values
//! assert_eq!(a | b | c, ordmask![0, 30]);
//! ```
//!
//! # Intersection
//!
//! ```
//! use ordmask::{OrdMask, ordmask};
//!
//! let a = ordmask![0, 15];
//! let b = ordmask![5, 20];
//! let c = ordmask![10, 30];
//! // &a & &b & &c: reference operators do not move (consume) the values
//! assert_eq!(&a & &b & &c, OrdMask::intersection(&[&a, &b, &c]));
//! // a & b & c: non-reference operators move (consume) the values
//! assert_eq!(a & b & c, ordmask![10, 15]);
//! ```
//!
//! # Minus and Complement
//!
//! ```
//! use ordmask::{OrdMask, ordmask};
//!
//! let a = ordmask![0, 15];
//! let b = ordmask![5, 8];
//! let c = ordmask![10, 20];
//! // &a - &b - &c: reference operators do not move (consume) the values
//! assert_eq!(&a - &b - &c, OrdMask::minus(&a, &[&b, &c]));
//! // a - b - c: non-reference operators move (consume) the values
//! assert_eq!(a - b - c, ordmask![0, 5, 8, 10]);
//!
//! let a = ordmask![0, 15];
//! // !&a: reference operator and `a.complement()` do not move (consume) the value
//! assert_eq!(!&a, a.complement());
//! // !a: non-reference operator and `a.to_complement()` move (consume) the value
//! assert_eq!(!a, ordmask![.., 0, 15]);
//! ```
//!
//! # Symmetric Difference
//!
//! ```
//! use ordmask::{OrdMask, ordmask};
//!
//! let a = ordmask![0, 15];
//! let b = ordmask![5, 20];
//! // &a ^ &b: reference operators do not move (consume) the values
//! assert_eq!(&a ^ &b, OrdMask::symmetric_difference(&a, &b));
//! // a ^ b: non-reference operators move (consume) the values
//! assert_eq!(a ^ b, ordmask![0, 5, 15, 20]);
//! ```
//!
//! # Spans
//!
//! [`OrdMask`] provides methods to iterate over included spans.
//! Each span is returned as a tuple `(start, end)` representing a half-open interval `[start, end)`.
//!
//! > **Note**:
//! > - Using spans requires type `T` to implement the [`WithMax`] trait
//! >   (the library provides implementations for all standard integer types).
//! > - Since spans are half-open intervals `[start, end)`, whether `MAX` is included can be confusing.
//! >   Use [`.is_max_value_included()`](OrdMask::is_max_value_included) to check if the maximum value is in the mask.
//!
//! ## Basic Iteration
//!
//! Use [`.spans()`](OrdMask::spans) to iterate over included spans.
//!
//! ```
//! use ordmask::ordmask;
//!
//! // Empty mask has no spans
//! assert_eq!(ordmask![<i32>].spans().collect::<Vec<_>>(), vec![]);
//!
//! // Universal mask has one span [MIN, MAX]
//! assert_eq!(
//!     ordmask![..].spans().collect::<Vec<_>>(),
//!     vec![(i32::MIN, i32::MAX)]
//! );
//!
//! // Single span [1, 2)
//! assert_eq!(ordmask![1, 2].spans().collect::<Vec<_>>(), vec![(1, 2)]);
//!
//! // Multiple spans: [MIN, 1) and [2, MAX]
//! assert_eq!(
//!     ordmask![.., 1, 2].spans().collect::<Vec<_>>(),
//!     vec![(i32::MIN, 1), (2, i32::MAX)]
//! );
//! ```
//!
//! ## Owning Iteration
//!
//! Use [`.into_spans()`](OrdMask::into_spans) to consume the mask and return an owning iterator:
//!
//! ```
//! use ordmask::ordmask;
//!
//! assert_eq!(
//!     ordmask![.., 1, 2_i32].into_spans().collect::<Vec<_>>(),
//!     vec![(i32::MIN, 1), (2, i32::MAX)]
//! );
//! ```
//!
//! ## Span Count and Size
//!
//! Use [`.spans_count()`](OrdMask::spans_count) to get the number of spans in **O(1)** time without consuming an iterator.
//! It's equivalent to [`.spans().count()`](Iterator::count) but more efficient.
//!
//! ```
//! use ordmask::{ordmask, spans::SumSize};
//!
//! // Span count
//! assert_eq!(ordmask![.., 10].spans_count(), 1);      // [MIN, 10)
//! assert_eq!(ordmask![.., 10, 20].spans_count(), 2);  // [MIN, 10), [20, MAX]
//! assert_eq!(ordmask![<u32>].spans_count(), 0);
//! assert_eq!(ordmask![<u32>..].spans_count(), 1);
//!
//! // Sum of span sizes
//! // [0, 10)
//! assert_eq!(ordmask![<u32> .., 10].spans().sum_size(), 10);
//! // [0, 10), [20, MAX]
//! assert_eq!(ordmask![<u32> .., 10, 20].spans().sum_size(), u32::MAX - 10 + 1);
//! // Empty mask has size 0
//! assert_eq!(ordmask![<u32>].spans().sum_size(), 0);
//! ```
//!
//! > **Warning**: [`.spans().sum_size()`](SumSize::sum_size) may panic due to overflow when called on a universal mask
//! > (because `MAX - MIN + 1` overflows).
//! > Use [`.is_universal()`](OrdMask::is_universal) to check before calling this.
//!
//! # Type Requirements
//!
//! [`OrdMask<T>`](OrdMask) requires `T` to implement the [`WithMin`] trait,
//! a trait for types that have a minimum value.
//! The library provides implementations for all standard integer types:
//!
//! ```
//! use ordmask::WithMin;
//!
//! // Built-in implementations for:
//! // u8, u16, u32, u64, u128, usize
//! // i8, i16, i32, i64, i128, isize
//!
//! assert_eq!(i32::MIN, <i32 as WithMin>::MIN);
//! assert_eq!(u64::MIN, <u64 as WithMin>::MIN);
//! ```
//!
//! To use custom types:
//! - At minimum, implement [`WithMin`] to use [`OrdMask`]
//! - To use [`.spans()`](OrdMask::spans) or [`.into_spans()`](OrdMask::into_spans),
//!   also implement [`WithMax`]
//! - To use [`.spans().sum_size()`](SumSize::sum_size), also implement [`OrderedSub`]
//!
//! > **Note**: [`.spans_count()`](OrdMask::spans_count) is special—it only requires [`WithMin`], not [`WithMax`].
//!
//! ```
//! use ordmask::{OrderedSub, WithMax, WithMin, ordmask, spans::SumSize};
//!
//! #[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
//! struct MyType(i32);
//!
//! // Required implementation, Enables `.spans_count()`
//! impl WithMin for MyType {
//!     const MIN: Self = MyType(i32::MIN);
//! }
//!
//! // Enables `.spans()`, `.into_spans()`
//! impl WithMax for MyType {
//!     const MAX: Self = MyType(i32::MAX);
//! }
//!
//! // Enables `.spans().sum_size()`
//! impl OrderedSub for MyType {
//!     type Target = u32;
//!
//!     fn ordered_sub(&self, other: &Self) -> Self::Target {
//!         self.0.ordered_sub(&other.0) // Same as the library does for i32
//!     }
//! }
//!
//! assert!(ordmask![..].included(&MyType(1)));
//! assert_eq!(ordmask![MyType(0), MyType(10)].spans().sum_size(), 10);
//! ```

mod macros;
mod ops;

mod error;
mod ordmask;
mod traits;

pub use error::*;
pub use ordmask::*;
pub use traits::*;

pub mod utils;
