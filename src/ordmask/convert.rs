use crate::{Error, OrdMask, WithMin};

impl<T: Ord + Clone + WithMin> From<OrdMask<T>> for Vec<T> {
    fn from(mask: OrdMask<T>) -> Self {
        mask.key_points
    }
}

impl<T: Ord + Clone + WithMin> TryFrom<Vec<T>> for OrdMask<T> {
    type Error = Error;

    /// Create an [`OrdMask`] from empty with `key_points`.
    ///
    /// Equivalent to `OrdMask::try_new(..., false)`. See [`OrdMask::try_new`].
    fn try_from(key_points: Vec<T>) -> Result<Self, Self::Error> {
        Self::try_new(key_points, false)
    }
}

impl<T: Ord + Clone + WithMin> OrdMask<T> {
    /// Create an [`OrdMask`] with key points.
    ///
    /// - `key_points`: values where the mask actually changes state (must be non-decreasing)
    /// - `based_on_universal`: if `true`, the mask starts from universal; otherwise from empty
    ///
    /// Unlike [`OrdMask::from_suspicious_points_set`], `key_points` should contain only the
    /// exact points where state transitions occur, not merely potential transition points.
    ///
    /// # Example
    ///
    /// For `OrdMask::try_new(vec![1, 4], false)`:
    ///
    /// | ✕ | ✕ | ✓ | ✓ | ✓ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// Can be constructed by starting from empty and switching to included at 1,
    /// then to excluded at 4.
    ///
    /// For `OrdMask::try_new(vec![2], true)`:
    ///
    /// | ✓ | ✓ | ✓ | ✕ | ✕ | ✕ | ✕ |
    /// |---|---|---|---|---|---|---|
    /// |...| 0 | 1 | 2 | 3 | 4 |...|
    ///
    /// Can be constructed by starting from universal and switching to excluded at 2.
    ///
    /// # Errors
    ///
    /// Returns an error if key points are not non-decreasing.
    pub fn try_new(key_points: Vec<T>, based_on_universal: bool) -> Result<Self, Error> {
        match crate::utils::is_increasing::<false, _>(&key_points) {
            (true, _) => {
                let mut result = Self {
                    key_points,
                    based_on_universal,
                };
                result.simplify();
                Ok(result)
            }
            (false, n) => Err(Error::FallingAt(n)),
        }
    }

    /// Create an [`OrdMask`] from empty with `key_points`.
    ///
    /// Equivalent to `OrdMask::try_new(..., false).unwrap()`. See [`OrdMask::try_new`].
    ///
    /// # Panics
    ///
    /// Panics if key points are not in non-decreasing order.
    /// Use [`Self::try_from`] for a non-panicking alternative.
    pub fn from(key_points: Vec<T>) -> Self {
        Self::try_new(key_points, false).unwrap()
    }

    /// Create an [`OrdMask`] from universal with `key_points`.
    ///
    /// Equivalent to `OrdMask::try_new(..., true).unwrap()`. See [`OrdMask::try_new`].
    ///
    /// # Panics
    ///
    /// Panics if key points are not in non-decreasing order.
    pub fn from_complement(key_points: Vec<T>) -> Self {
        Self::try_new(key_points, true).unwrap()
    }

    /// Create an [`OrdMask`] with key points without validation.
    ///
    /// See safe version [`OrdMask::try_new`].
    ///
    /// # Safety
    ///
    /// Key points must be strictly increasing and contain no duplicates.
    pub const unsafe fn with_checked(key_points: Vec<T>, based_on_universal: bool) -> Self {
        Self {
            key_points,
            based_on_universal,
        }
    }
}
