mod construct;
mod convert;
mod operations;
mod ops;

/// An `OrdMask` can be used to check if a value is included.
///
/// It is a list of values in ascending order and a pair in two-element tuples means a included range.
///
/// For example, the mask `[0, 2, 4]` means that `[0, 2)` and `[4, \infty)` are included,  
/// and the values in `(-\infty, 0)` and `[2, 4)` are excluded.
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
/// let mask = ordmask![_, 0, 10, 20];
/// (-10..0).for_each(|x| should_include(&mask, x));
/// (0..10).for_each(|x| should_exclude(&mask, x));
/// (10..20).for_each(|x| should_include(&mask, x));
/// (20..30).for_each(|x| should_exclude(&mask, x));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrdMask<T: Ord + Clone> {
    key_points: Vec<T>,
    reversed: bool,
}

impl<T: Ord + Clone> OrdMask<T> {
    /// Check if the `OrdMask` is empty.
    ///
    /// An empty `OrdMask` means no value is included.
    pub fn is_empty(&self) -> bool {
        !self.reversed && self.key_points.is_empty()
    }

    /// Check if the mask is universal.
    ///
    /// An universal mask includes all values.
    pub fn is_universal(&self) -> bool {
        self.key_points.is_empty() && self.reversed
    }

    /// Check if the `OrdMask` is valid.
    ///
    /// There is no need to test `is_valid()` if you never use unsafe methods.
    pub fn is_valid(&self) -> bool {
        convert::get_first_falling_index(&self.key_points) == 0
    }

    /// Check if a value is included in this mask.
    pub fn included(&self, value: &T) -> bool {
        self.reversed ^ (self.key_points.partition_point(|x| x <= value) % 2 == 1)
    }

    /// Check if a value is excluded in this mask.
    pub fn excluded(&self, value: &T) -> bool {
        !self.included(value)
    }

    /// Check if the `OrdMask` includes the maximum value.
    pub fn is_include_max_value(&self) -> bool {
        self.reversed ^ (self.key_points.len() % 2 == 1)
    }

    /// Check if the `OrdMask` includes the minimum value.
    pub fn is_include_min_value(&self) -> bool {
        self.reversed
    }

    /// Check if the `OrdMask` is simplified.
    ///
    /// An simplified `OrdMask` means there are no duplicate values.
    ///
    /// There is no need to test `is_simplified()` if you never use unsafe methods,  
    /// otherwise you should directly use `simplify()` without checking `is_simplified()`.
    pub fn is_simplified(&self) -> bool {
        for i in 1..self.key_points.len() {
            if self.key_points[i] == self.key_points[i - 1] {
                return false;
            }
        }
        true
    }

    /// Remove meaningless values in the `OrdMask`.
    ///
    /// The safe methods will ensure the `OrdMask` is simplified automatically.  
    /// Therefore, there is no need to call it unless you are using unsafe methods.
    ///
    /// # Examples
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    ///
    /// let mut mask = ordmask![0, 0, 1, 1];
    /// assert_eq!(mask, ordmask![]);
    ///
    /// let mut mask = ordmask![0, 2, 2, 2, 3, 3, 4];
    /// assert_eq!(mask, ordmask![0, 2, 4]);
    ///
    /// let mut mask = unsafe { OrdMask::with_unchecked(vec![0, 2, 2, 2, 4, 4, 4, 6, 6, 8, 8], false) };
    /// mask.simplify();
    /// assert_eq!(mask, ordmask![0, 2, 4]);
    /// ```
    pub fn simplify(&mut self) {
        let len = self.key_points.len();
        if len < 2 {
            return;
        }

        let mut write_index = 0;
        let mut read_index = 0;
        while read_index < len {
            let start = read_index;
            while read_index < len && self.key_points[read_index] == self.key_points[start] {
                read_index += 1;
            }
            if (read_index - start) % 2 == 1 {
                self.key_points[write_index] = self.key_points[start].clone();
                write_index += 1;
            }
        }

        self.key_points.truncate(write_index);
    }

    pub fn key_points(&self) -> &Vec<T> {
        &self.key_points
    }

    /// Get the key points of `masks`.
    pub fn get_key_points_set(masks: &[&OrdMask<T>]) -> std::collections::BTreeSet<T> {
        let mut result = std::collections::BTreeSet::new();
        for item in masks {
            result.extend(item.key_points.iter().cloned());
        }
        result
    }
}
