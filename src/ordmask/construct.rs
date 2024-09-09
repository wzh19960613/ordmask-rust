use super::OrdMask;
use crate::MinValue;

impl<T: Ord + Clone> OrdMask<T> {
    /// Create an empty OrdMask which includes no values.
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Create a new OrdMask that includes all values greater than or equal to `value`.
    pub fn not_less_than(value: T) -> Self {
        Self(vec![value])
    }

    /// Create a new OrdMask that includes all values in the range `[start, end)`.
    pub fn in_range(start: T, end: T) -> Self {
        match start < end {
            true => Self(vec![start, end]),
            false => Self::empty(),
        }
    }

    /// Create a new OrdMask from a set of key points and a predicate.
    ///
    /// The key points are the values where the mask changes its state.  
    /// `is_included` is used to test if a key point is included or excluded in the mask.
    ///
    /// For example, a mask like this:
    ///
    /// | ✕ | ✕ | ✓ | ✓ | ✓ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// must contain key points `1` and `4`,
    /// and `is_included(1)` must be `true` and `is_included(4)` must be `false`.
    ///
    /// If you need a mask include all values less than some value, the key points
    /// must contain the minimum value, and `is_included(T::min_value())` must be `true`.
    /// For example, a mask like this:
    ///
    /// | ✓ | ✓ | ✓ | ✕ | ✕ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// must contain key points `i32::min_value()` and `2`,
    /// and `is_included(i32::min_value())` must be `true` and `is_included(2)` must be `false`.
    pub fn new(
        key_points: std::collections::BTreeSet<T>,
        mut is_included: impl FnMut(&T) -> bool,
    ) -> Self {
        let mut mask = Vec::with_capacity(key_points.len());
        for point in key_points {
            if is_included(&point) == (mask.len() % 2 == 0) {
                mask.push(point);
            }
        }
        mask.shrink_to_fit();
        Self(mask)
    }

    /// Create a new OrdMask from a key points map.
    ///
    /// The key points map is a map of key points and their corresponding boolean values.
    /// The boolean value indicates whether the key point is included in the mask.
    ///
    /// For example, a mask like this:
    ///
    /// | ✕ | ✕ | ✓ | ✓ | ✓ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// the key points map must include `(1, true)` and `(4, false)`.
    ///
    /// If you need a mask include all values less than some value,
    /// the key points map must contain `(T::min_value(), true)`.
    /// For example, a mask like this:
    ///
    /// | ✓ | ✓ | ✓ | ✕ | ✕ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// the key points map must include `(i32::min_value(), true)` and `(2, false)`.
    pub fn from_key_points_map(map: std::collections::BTreeMap<T, bool>) -> Self {
        let mut mask = Vec::with_capacity(map.len());
        for (point, is_included) in map {
            if is_included == (mask.len() % 2 == 0) {
                mask.push(point);
            }
        }
        mask.shrink_to_fit();
        Self(mask)
    }
}

impl<T: Ord + Clone + MinValue> OrdMask<T> {
    /// Create a new universal OrdMask which includes all values.
    pub fn universal() -> Self {
        Self(vec![T::min_value()])
    }

    /// Create a new OrdMask that includes all values less than `value`.
    pub fn less_than(value: T) -> Self {
        match value == T::min_value() {
            true => Self::empty(),
            false => Self(vec![T::min_value(), value]),
        }
    }
}
