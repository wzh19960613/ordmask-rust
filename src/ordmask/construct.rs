use super::OrdMask;

impl<T: Ord + Clone> OrdMask<T> {
    fn new(key_points: Vec<T>, reversed: bool) -> Self {
        Self {
            key_points,
            reversed,
        }
    }

    /// Create an empty OrdMask which includes no values.
    ///
    /// It's the same as `ordmask![]`.
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    /// let mask: OrdMask<i32> = ordmask![];
    /// assert_eq!(mask, OrdMask::empty());
    /// ```
    pub fn empty() -> Self {
        Self::new(Vec::new(), false)
    }

    /// Create a new universal OrdMask which includes all values.
    ///
    /// It's the same as `ordmask![_]`.
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    /// let mask: OrdMask<u64> = ordmask![_];
    /// assert_eq!(mask, OrdMask::universal());
    /// ```
    pub fn universal() -> Self {
        Self::new(Vec::new(), true)
    }

    /// Create a new OrdMask that includes all values greater than or equal to `value`.
    ///
    /// It's the same as `ordmask![_, value]`.
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    /// assert_eq!(ordmask![1], OrdMask::not_less_than(1));
    /// ```
    pub fn not_less_than(value: T) -> Self {
        Self::new(vec![value], false)
    }

    /// Create a new OrdMask that includes all values less than `value`.
    ///
    /// It's the same as `ordmask![_, value]`.
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    /// assert_eq!(ordmask![_, 1], OrdMask::less_than(1));
    /// ```
    pub fn less_than(value: T) -> Self {
        Self::new(vec![value], true)
    }

    /// Create a new OrdMask that includes all values in the range `[start, end)`.
    ///
    /// It's the same as `ordmask![start, end]`.
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

    /// Create a new OrdMask that excludes all values in the range `[start, end)`.
    ///
    /// It's the same as `ordmask![_, start, end]`.
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    /// assert_eq!(ordmask![_, 1, 10], OrdMask::exclude_range(1, 10));
    /// ```
    pub fn exclude_range(start: T, end: T) -> Self {
        match start < end {
            true => Self::new(vec![start, end], true),
            false => Self::universal(),
        }
    }

    /// Create a new OrdMask from a set of key points and a predicate.
    ///
    /// The `key_points` are the values where the mask changes its state.
    ///
    /// The `is_included` is used to test if a key point is included or excluded in the mask.
    ///
    /// If you need a mask include all values less than some value,
    /// the `include_min_value` must be `true`.
    /// Otherwise, the `include_min_value` must be `false`.
    ///
    /// For example, a mask like this:
    ///
    /// | ✕ | ✕ | ✓ | ✓ | ✓ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// must contain key points `1` and `4`,
    /// and `is_included(1)` must be `true` and `is_included(4)` must be `false`,
    /// and the `include_min_value` must be `false`.
    ///
    /// A mask like this:
    ///
    /// | ✓ | ✓ | ✓ | ✕ | ✕ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// must contain key points `2`,
    /// and `is_included(2)` must be `false`,
    /// and the `include_min_value` must be `true`.
    pub fn from_key_points_set(
        key_points: std::collections::BTreeSet<T>,
        is_included: impl Fn(&T) -> bool,
        include_min_value: bool,
    ) -> Self {
        let mut mask = Vec::with_capacity(key_points.len());
        for point in key_points {
            if (is_included(&point) == (mask.len() % 2 == 0)) ^ include_min_value {
                mask.push(point);
            }
        }
        mask.shrink_to_fit();
        Self::new(mask, include_min_value)
    }

    /// Create a new OrdMask from a key points to boolean map.
    ///
    /// The `map` is a map of key points to the boolean value indicates
    /// whether the key point is included in the mask.
    ///
    /// If you need a mask include all values less than some value,
    /// the `include_min_value` must be `true`.
    /// Otherwise, the `include_min_value` must be `false`.
    ///
    /// For example, a mask like this:
    ///
    /// | ✕ | ✕ | ✓ | ✓ | ✓ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// the key points map must include `(1, true)` and `(4, false)`,
    /// and the `include_min_value` must be `false`.
    ///
    /// A mask like this:
    ///
    /// | ✓ | ✓ | ✓ | ✕ | ✕ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// the key points map must include `(2, false)`,
    /// and the `include_min_value` must be `true`.
    pub fn from_key_points_map(
        map: std::collections::BTreeMap<T, bool>,
        include_min_value: bool,
    ) -> Self {
        let mut mask = Vec::with_capacity(map.len());
        for (point, is_included) in map {
            if (is_included == (mask.len() % 2 == 0)) ^ include_min_value {
                mask.push(point);
            }
        }
        mask.shrink_to_fit();
        Self::new(mask, include_min_value)
    }
}

/// Create an `OrdMask` from a list of key points.
///
/// # Panics
///
/// It will panic if the key points are not non-decreasing.
///
/// # Examples
///
/// ```
/// use ordmask::{OrdMask, ordmask};
///
/// let mask: OrdMask<i32> = ordmask![];
/// assert_eq!(mask, OrdMask::empty());
///
/// let mask: OrdMask<u64> = ordmask![_];
/// assert_eq!(mask, OrdMask::universal());
///
/// let mask = ordmask![0, 10, 20];
/// assert_eq!(mask, OrdMask::from(vec![0, 10, 20]));
///
/// let mask = ordmask![_, 0, 10, 20];
/// assert_eq!(mask, OrdMask::from_complement(vec![0, 10, 20]));
/// ```
#[macro_export]
macro_rules! ordmask {
    () => {
        ordmask::OrdMask::empty()
    };
    ($($key_points:expr),+ $(,)?) => {
        ordmask::OrdMask::from(vec![$($key_points),+])
    };
    (_, $($key_points:expr),+ $(,)?) => {
        ordmask::OrdMask::from_complement(vec![$($key_points),+])
    };
    (_) => {
        ordmask::OrdMask::universal()
    };
}

/// Create an `OrdMask` from a list of key points without checking if the key points are non-decreasing.
///
/// # Safety
///
/// Make sure the key points are non-decreasing, otherwise the behavior is undefined.
#[macro_export]
macro_rules! ordmask_uncheck {
    () => {
        ordmask::OrdMask::empty()
    };
    ($($key_points:expr),+ $(,)?) => {
        ordmask::OrdMask::with_unchecked(vec![$($key_points),+], false)
    };
    (_, $($key_points:expr),+ $(,)?) => {
        ordmask::OrdMask::with_unchecked(vec![$($key_points),+], true)
    };
    (_) => {
        ordmask::OrdMask::universal()
    };
}
