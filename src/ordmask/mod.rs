use std::collections::BTreeSet;

mod construct;
mod convert;
mod error;
mod macros;
mod operations;
mod ops;

pub mod iter;

pub use error::*;

use super::WithMin;

/// A mask for efficiently checking if a value is included.
///
/// An `OrdMask` stores key points in ascending order. Consecutive key points define
/// alternating included/excluded intervals.
///
/// For example, `ordmask![0, 2, 4]` includes `[0, 2)` and `[4, MAX]`,
/// and excludes `[MIN, 0)` and `[2, 4)`.
///
/// Use [`ordmask!`](crate::ordmask!) to create an `OrdMask`.
///
/// # Examples
/// ```
/// use ordmask::{ordmask, OrdMask};
///
/// fn should_include(mask: &OrdMask<i32>, x: i32) {
///     assert!(mask.included(&x), "should include {}", x);
/// }
///
/// fn should_exclude(mask: &OrdMask<i32>, x: i32) {
///     assert!(mask.excluded(&x), "should exclude {}", x);
/// }
///
/// let mask = ordmask![];
/// assert!(mask.is_empty());
/// (-10..10).for_each(|x| should_exclude(&mask, x));
///
/// let mask = ordmask![0];
/// (-10..0).for_each(|x| should_exclude(&mask, x));
/// (0..10).for_each(|x| should_include(&mask, x));
///
/// let mask = ordmask![0, 10];
/// (-10..0).for_each(|x| should_exclude(&mask, x));
/// (0..10).for_each(|x| should_include(&mask, x));
/// (10..20).for_each(|x| should_exclude(&mask, x));
///
/// let mask = ordmask![0, 10, 20];
/// (-10..0).for_each(|x| should_exclude(&mask, x));
/// (0..10).for_each(|x| should_include(&mask, x));
/// (10..20).for_each(|x| should_exclude(&mask, x));
/// (20..30).for_each(|x| should_include(&mask, x));
///
/// let mask = ordmask![.., 0, 10, 20];
/// (-10..0).for_each(|x| should_include(&mask, x));
/// (0..10).for_each(|x| should_exclude(&mask, x));
/// (10..20).for_each(|x| should_include(&mask, x));
/// (20..30).for_each(|x| should_exclude(&mask, x));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OrdMask<T: Ord + Clone + WithMin> {
    key_points: Vec<T>,
    based_on_universal: bool,
}

impl<T: Ord + Clone + WithMin> AsRef<OrdMask<T>> for OrdMask<T> {
    fn as_ref(&self) -> &OrdMask<T> {
        self
    }
}

impl<T: Ord + Clone + WithMin> OrdMask<T> {
    /// Check if the `OrdMask` is empty.
    ///
    /// An empty `OrdMask` means no value is included.
    pub const fn is_empty(&self) -> bool {
        !self.based_on_universal && self.key_points.is_empty()
    }

    /// Check if the mask is universal (includes all values).
    pub const fn is_universal(&self) -> bool {
        self.based_on_universal && self.key_points.is_empty()
    }

    /// Check if the `OrdMask` is valid.
    ///
    /// Unnecessary if you never use unsafe methods.
    pub fn is_valid(&self) -> bool {
        crate::utils::is_increasing::<true, _>(&self.key_points).0
    }

    /// Returns whether this mask is based on a universal mask (`true`) or an empty mask (`false`).
    ///
    /// `ordmask![1]` and `ordmask![.., 1]` have the same key points, but:
    /// - `ordmask![1]` starts from empty, includes `[1, MAX]`
    /// - `ordmask![.., 1]` starts from universal, includes `[MIN, 1)`
    pub const fn based_on_universal(&self) -> &bool {
        &self.based_on_universal
    }

    /// Mutable reference to [`based_on_universal`](OrdMask::based_on_universal).
    ///
    /// Changing this has the same effect as calling [`.reverse()`](OrdMask::reverse).
    /// For most cases, you should use [`.reverse()`](OrdMask::reverse) instead.
    pub const fn mut_based_on_universal(&mut self) -> &mut bool {
        &mut self.based_on_universal
    }

    /// Check if a value is included in this mask.
    /// Equivalent to [`.contains(...)`](OrdMask::contains).
    pub fn included(&self, value: &T) -> bool {
        let partition_point = self.key_points.partition_point(|x| x <= value);
        self.based_on_universal == partition_point.is_multiple_of(2)
    }

    /// Check if a value is excluded in this mask.
    /// Equivalent to **negating** [`.included(...)`](OrdMask::included).
    pub fn excluded(&self, value: &T) -> bool {
        !self.included(value)
    }

    /// Check if a value is contained in this mask.
    /// Equivalent to [`.included(...)`](OrdMask::included).
    pub fn contains(&self, value: &T) -> bool {
        self.included(value)
    }

    /// Returns the number of included spans.
    ///
    /// Unlike [`.spans().count()`](Iterator::count) from the standard library,
    /// this method is **O(1)** and does not consume or iterate over the spans.
    ///
    /// See also [`.spans()`](OrdMask::spans).
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::ordmask;
    /// assert_eq!(ordmask![.., 10].spans_count(), 1);        // [MIN, 10)
    /// assert_eq!(ordmask![.., 10, 20].spans_count(), 2);    // [MIN, 10), [20, MAX]
    /// assert_eq!(ordmask![<u32>].spans_count(), 0);         // Empty
    /// assert_eq!(ordmask![<u32>..].spans_count(), 1);       // [MIN, MAX]
    /// ```
    pub const fn spans_count(&self) -> usize {
        let delta = if self.based_on_universal { 2 } else { 1 };
        (delta + self.key_points.len()) / 2
    }

    /// Check if the maximum value is included.
    ///
    /// ```
    /// use ordmask::ordmask;
    ///
    /// assert!(ordmask![1].is_max_value_included());
    /// assert!(ordmask![<i32> ..].is_max_value_included());
    /// assert!(ordmask![.., 1, 2].is_max_value_included());
    /// assert!(!ordmask![1, 2].is_max_value_included());
    /// ```
    pub const fn is_max_value_included(&self) -> bool {
        self.based_on_universal == self.key_points.len().is_multiple_of(2)
    }

    /// Simplify the mask by removing redundant key points.
    ///
    /// Unnecessary if you never use unsafe constructors, as safe methods
    /// ensure the mask is always simplified.
    ///
    /// Returns `true` if the mask was modified.
    ///
    /// # Examples
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    ///
    /// let mut mask = unsafe { OrdMask::with_checked(vec![0, 0], false) };
    /// mask.simplify();
    /// assert_eq!(mask, ordmask![]);
    ///
    /// let mut mask = unsafe { OrdMask::<u32>::with_checked(vec![0], true) };
    /// mask.simplify();
    /// assert_eq!(mask, ordmask![]);
    /// ```
    pub fn simplify(&mut self) -> bool {
        let len = match self.key_points.len() {
            0 => return false,
            n => n,
        };
        let mut write_index = 0;
        let mut read_index = 0;

        #[inline(always)]
        fn move_to_next_value<T: PartialEq>(idx: &mut usize, arr: &[T], len: usize, now_value: &T) {
            while *idx < len && &arr[*idx] == now_value {
                *idx += 1;
            }
        }

        move_to_next_value(&mut read_index, &self.key_points, len, &T::MIN);
        if read_index != 0 && !read_index.is_multiple_of(2) {
            self.based_on_universal = !self.based_on_universal;
        }

        while read_index < len {
            let (start_index, now_value) = (read_index, self.key_points[read_index].clone());
            read_index += 1;
            move_to_next_value(&mut read_index, &self.key_points, len, &now_value);
            if !(read_index - start_index).is_multiple_of(2) {
                self.key_points[write_index] = now_value;
                write_index += 1;
            }
        }
        self.key_points.truncate(write_index);
        write_index < len
    }

    /// Reference to the key points of this mask.
    pub const fn key_points(&self) -> &Vec<T> {
        &self.key_points
    }

    /// Mutable reference to the key points of this mask.
    ///
    /// # Safety
    ///
    /// Must ensure that the key_points are unique and strictly increasing after modification.
    pub const unsafe fn mut_key_points(&mut self) -> &mut Vec<T> {
        &mut self.key_points
    }

    /// Get suspicious points from multiple masks.
    ///
    /// Includes all key points, plus `T::MIN`
    /// if any mask has [`.based_on_universal()`](OrdMask::based_on_universal) = `true`.
    pub fn get_suspicious_points<I>(masks: I) -> BTreeSet<T>
    where
        I: IntoIterator,
        I::Item: AsRef<OrdMask<T>>,
    {
        let mut result = BTreeSet::new();
        let mut has_universal = false;
        for item in masks {
            let item = item.as_ref();
            result.extend(item.key_points.clone());
            if item.based_on_universal {
                has_universal = true;
            }
        }
        if has_universal {
            result.insert(T::MIN);
        }
        result
    }
}
