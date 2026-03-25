use std::collections::{BTreeMap, BTreeSet};

use crate::{OrdMask, WithMin};

impl<T: Ord + Clone + WithMin> OrdMask<T> {
    const fn new(key_points: Vec<T>, reversed: bool) -> Self {
        Self {
            key_points,
            based_on_universal: reversed,
        }
    }

    /// Create an empty [`OrdMask`] (includes no values).
    ///
    /// Equivalent to [`ordmask![]`](crate::ordmask!).
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    /// let mask: OrdMask<i32> = ordmask![];
    /// assert_eq!(mask, OrdMask::empty());
    /// ```
    pub const fn empty() -> Self {
        Self::new(Vec::new(), false)
    }

    /// Create a universal [`OrdMask`] (includes all values).
    ///
    /// Equivalent to [`ordmask![..]`](crate::ordmask!).
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    /// let mask: OrdMask<u64> = ordmask![..];
    /// assert_eq!(mask, OrdMask::universal());
    /// ```
    pub const fn universal() -> Self {
        Self::new(Vec::new(), true)
    }

    /// Create an [`OrdMask`] that includes all values `≥ value`.
    ///
    /// Equivalent to [`ordmask![value]`](crate::ordmask!).
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    /// assert_eq!(ordmask![1], OrdMask::not_less_than(1));
    /// ```
    pub fn not_less_than(value: T) -> Self {
        Self::new(vec![value], false)
    }

    /// Create an [`OrdMask`] that includes all values `< value`.
    ///
    /// Equivalent to [`ordmask![.., value]`](crate::ordmask!).
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    /// assert_eq!(ordmask![.., 1], OrdMask::less_than(1));
    /// ```
    pub fn less_than(value: T) -> Self {
        Self::new(vec![value], true)
    }

    /// Create an [`OrdMask`] that includes values in `[start, end)`.
    ///
    /// Equivalent to [`ordmask![start, end]`](crate::ordmask!).
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    /// assert_eq!(ordmask![1, 10], OrdMask::in_range(1, 10));
    /// ```
    pub fn in_range(start: T, end: T) -> Self {
        match start < end {
            true => Self::new(vec![start, end], false),
            false => Self::empty(),
        }
    }

    /// Create an [`OrdMask`] that excludes values in `[start, end)`.
    ///
    /// Equivalent to [`ordmask![.., start, end]`](crate::ordmask!).
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    /// assert_eq!(ordmask![.., 1, 10], OrdMask::exclude_range(1, 10));
    /// ```
    pub fn exclude_range(start: T, end: T) -> Self {
        match start < end {
            true => Self::new(vec![start, end], true),
            false => Self::universal(),
        }
    }

    /// Create an [`OrdMask`] from suspicious points and an inclusion predicate.
    ///
    /// - `suspicious_points`: values where the mask may change state
    /// - `is_included`: a function that returns `true`
    ///   if a suspicious point is included in the mask, `false` otherwise
    /// - `based_on_universal`: if `true`, the mask starts from universal; otherwise from empty
    ///
    /// # Example
    ///
    /// For a mask including `[1, 4)`:
    ///
    /// | ✕ | ✕ | ✓ | ✓ | ✓ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// Can be constructed by starting from empty and
    /// switching to included at 1, then to excluded at 4. Therefore:
    /// - `suspicious_points` must include at least 1 and 4
    /// - `is_included(suspicious_point)` must return `true` if `suspicious_point` is in `[1, 4)`,
    ///   and `false` otherwise
    /// - `based_on_universal` should be `false`.
    ///
    /// For a mask including `[MIN, 2)`:
    ///
    /// | ✓ | ✓ | ✓ | ✕ | ✕ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// Can be constructed by starting from universal and switching to excluded at 2. Therefore:
    /// - `suspicious_points` must include at least 2
    /// - `is_included(suspicious_point)` must return `true` if `suspicious_point` is in `[MIN, 2)`,
    ///   and `false` otherwise
    /// - `based_on_universal` should be `true`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use ordmask::{OrdMask, ordmask};
    ///
    /// let set = BTreeSet::from([1, 4]); // can include more values
    /// let mask = OrdMask::from_suspicious_points_set(set, |v| matches!(v, 1..4), false);
    /// assert_eq!(mask, ordmask![1, 4]);
    /// ```
    pub fn from_suspicious_points_set(
        suspicious_points: BTreeSet<T>,
        is_included: impl Fn(&T) -> bool,
        based_on_universal: bool,
    ) -> Self {
        unsafe { Self::from_suspicious_points(suspicious_points, is_included, based_on_universal) }
    }

    /// Create an [`OrdMask`] from suspicious points and an inclusion predicate.
    ///
    /// - `suspicious_points`: values where the mask may change state (must be strictly increasing)
    /// - `is_included`: a function that returns `true`
    ///   if a suspicious point is included in the mask, `false` otherwise
    /// - `based_on_universal`: if `true`, the mask starts from universal; otherwise from empty
    ///
    /// See the safe version [`from_suspicious_points_set`](OrdMask::from_suspicious_points_set).
    ///
    /// # Example
    ///
    /// For a mask including `[1, 4)`:
    ///
    /// | ✕ | ✕ | ✓ | ✓ | ✓ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// Can be constructed by starting from empty and
    /// switching to included at 1, then to excluded at 4. Therefore:
    /// - `suspicious_points` must include at least 1 and 4
    /// - `is_included(suspicious_point)` must return `true` if `suspicious_point` is in `[1, 4)`,
    ///   and `false` otherwise
    /// - `based_on_universal` should be `false`.
    ///
    /// For a mask including `[MIN, 2)`:
    ///
    /// | ✓ | ✓ | ✓ | ✕ | ✕ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// Can be constructed by starting from universal and switching to excluded at 2. Therefore:
    /// - `suspicious_points` must include at least 2
    /// - `is_included(suspicious_point)` must return `true` if `suspicious_point` is in `[MIN, 2)`,
    ///   and `false` otherwise
    /// - `based_on_universal` should be `true`.
    ///
    /// # Example
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    ///
    /// let points = [1, 4]; // can include more values, must be strictly increasing
    /// let mask = unsafe { OrdMask::from_suspicious_points(points, |v| matches!(v, 1..4), false) };
    /// assert_eq!(mask, ordmask![1, 4]);
    /// ```
    ///
    /// # Safety
    ///
    /// `suspicious_points` must be strictly increasing with no duplicates.
    pub unsafe fn from_suspicious_points(
        suspicious_points: impl IntoIterator<Item = T>,
        is_included: impl Fn(&T) -> bool,
        based_on_universal: bool,
    ) -> Self {
        let pairs = suspicious_points.into_iter().map(|v| {
            let is_included = is_included(&v);
            (v, is_included)
        });
        unsafe { Self::from_suspicious_points_pairs(pairs, based_on_universal, None) }
    }

    /// Construct an [`OrdMask`] from a map of suspicious points to inclusion states.
    ///
    /// - Key: the point where state may change
    /// - Value: `true` if included at that point, `false` if excluded
    /// - `based_on_universal`: if `true`, the mask starts from universal; otherwise from empty
    ///
    /// # Example
    ///
    /// For a mask including `[1, 4)`:
    ///
    /// | ✕ | ✕ | ✓ | ✓ | ✓ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// Can be constructed by starting from empty and
    /// switching to included at 1, then to excluded at 4. Therefore:
    /// - `map` must include `(1, true)` and `(4, false)`
    /// - `based_on_universal` should be `false`.
    ///
    /// For a mask including `[MIN, 2)`:
    ///
    /// | ✓ | ✓ | ✓ | ✕ | ✕ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// Can be constructed by starting from universal and switching to excluded at 2. Therefore:
    /// - `map` must include `(2, false)`
    /// - `based_on_universal` should be `true`.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::BTreeMap;
    /// use ordmask::{OrdMask, ordmask};
    ///
    /// let map = BTreeMap::from([(1, true), (4, false)]);
    /// let mask = OrdMask::from_suspicious_points_map(map, false);
    /// assert_eq!(mask, ordmask![1, 4]);
    /// ```
    pub fn from_suspicious_points_map(map: BTreeMap<T, bool>, based_on_universal: bool) -> Self {
        unsafe { Self::from_suspicious_points_pairs(map, based_on_universal, None) }
    }

    /// Construct an [`OrdMask`] from key-value pairs of suspicious points and inclusion states.
    ///
    /// - Key: the point where state may change (must be strictly increasing)
    /// - Value: `true` if included at that point, `false` if excluded
    /// - `based_on_universal`: if `true`, the mask starts from universal; otherwise from empty
    /// - `size_hint`: if provided, pre-allocates that capacity
    ///
    /// See the safe version [`from_suspicious_points_map`](OrdMask::from_suspicious_points_map).
    ///
    /// # Example
    ///
    /// For a mask including `[1, 4)`:
    ///
    /// | ✕ | ✕ | ✓ | ✓ | ✓ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// Can be constructed by starting from empty and
    /// switching to included at 1, then to excluded at 4. Therefore:
    /// - `pairs` must include `(1, true)` and `(4, false)`
    /// - `based_on_universal` should be `false`.
    ///
    /// For a mask including `[MIN, 2)`:
    ///
    /// | ✓ | ✓ | ✓ | ✕ | ✕ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// Can be constructed by starting from universal and switching to excluded at 2. Therefore:
    /// - `pairs` must include `(2, false)`
    /// - `based_on_universal` should be `true`.
    ///
    /// # Example
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    ///
    /// let pairs = [(1, true), (4, false)];
    /// let mask = unsafe { OrdMask::from_suspicious_points_pairs(pairs, false, None) };
    /// assert_eq!(mask, ordmask![1, 4]);
    /// ```
    ///
    /// # Safety
    ///
    /// Keys must be strictly increasing with no duplicates.
    pub unsafe fn from_suspicious_points_pairs(
        pairs: impl IntoIterator<Item = (T, bool)>,
        reversed: bool,
        size_hint: Option<usize>,
    ) -> Self {
        let mut iter = pairs.into_iter();
        let mut mask = Vec::with_capacity(size_hint.unwrap_or_else(|| iter.size_hint().0));
        let mut reversed = reversed;
        if let Some((first_point, first_is_included)) = iter.next() {
            if reversed != first_is_included {
                match T::MIN == first_point {
                    true => reversed = !reversed,
                    false => mask.push(first_point),
                }
            }
            for (point, is_included) in iter {
                if (is_included == mask.len().is_multiple_of(2)) != reversed {
                    mask.push(point);
                }
            }
        }
        mask.shrink_to_fit();
        Self::new(mask, reversed)
    }
}
