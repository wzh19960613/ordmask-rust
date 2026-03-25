use super::{OrdMask, WithMin};

impl<T: Ord + Clone + WithMin> OrdMask<T> {
    /// Create a new [`OrdMask`] representing the union of `masks`.
    ///
    /// A value is included in the result if it's included in at least one mask.
    ///
    /// `OrdMask::union(&[&a, &b, &c])` has the same result as `&a | &b | &c`,
    /// but this function is more efficient when operating on more than two masks.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    ///
    /// let mask1 = ordmask![0, 6];
    /// let mask2 = ordmask![5, 10];
    /// let mask3 = ordmask![20, 30];
    /// let union = OrdMask::union(&[&mask1, &mask2, &mask3]);
    /// assert_eq!(union, ordmask![0, 10, 20, 30]);
    /// ```
    pub fn union(masks: &[impl AsRef<OrdMask<T>>]) -> Self {
        Self::from_suspicious_points_set(
            Self::get_suspicious_points(masks),
            |x| masks.iter().any(|item| item.as_ref().included(x)),
            masks.iter().any(|item| item.as_ref().based_on_universal),
        )
    }

    /// See [`OrdMask::union`].
    pub fn union_from_iter(masks: impl IntoIterator<Item = impl AsRef<OrdMask<T>>>) -> Self {
        Self::union(masks.into_iter().collect::<Vec<_>>().as_slice())
    }

    /// Create a new [`OrdMask`] representing the intersection of `masks`.
    ///
    /// A value is included in the result if it's included in all masks.
    ///
    /// `OrdMask::intersection(&[&a, &b, &c])` has the same result as `&a & &b & &c`,
    /// but this function is more efficient when operating on more than two masks.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::{OrdMask, ordmask};
    ///
    /// let mask1 = ordmask![0, 6];
    /// let mask2 = ordmask![5, 10];
    /// let mask3 = ordmask![-10, 30];
    /// let intersection = OrdMask::intersection(&[&mask1, &mask2, &mask3]);
    /// assert_eq!(intersection, ordmask![5, 6]);
    /// ```
    pub fn intersection(masks: &[impl AsRef<OrdMask<T>>]) -> Self {
        Self::from_suspicious_points_set(
            Self::get_suspicious_points(masks),
            |x| masks.iter().all(|item| item.as_ref().included(x)),
            masks.iter().all(|item| item.as_ref().based_on_universal),
        )
    }

    /// See [`OrdMask::intersection`].
    pub fn intersection_from_iter(masks: impl IntoIterator<Item = impl AsRef<OrdMask<T>>>) -> Self {
        Self::intersection(masks.into_iter().collect::<Vec<_>>().as_slice())
    }

    /// Create a new [`OrdMask`] representing `self` minus `others`.
    ///
    /// A value is included in the result if it's included in `self`
    /// and excluded in all of `others`.
    ///
    /// `a.minus(&[&b, &c])` has the same result as `&a - &b - &c`,
    /// but this function is more efficient when operating on more than one mask in `others`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::ordmask;
    ///
    /// let mask1 = ordmask![0, 30];
    /// let mask2 = ordmask![5, 10];
    /// let mask3 = ordmask![6, 20];
    /// let difference = mask1.minus(&[&mask2, &mask3]);
    /// assert_eq!(difference, ordmask![0, 5, 20, 30]);
    /// ```
    pub fn minus(&self, others: &[impl AsRef<OrdMask<T>>]) -> Self {
        Self::from_suspicious_points_set(
            Self::get_suspicious_points(others.iter().map(|item| item.as_ref()).chain([self])),
            |x| self.included(x) && others.iter().all(|item| item.as_ref().excluded(x)),
            self.based_on_universal && !others.iter().any(|item| item.as_ref().based_on_universal),
        )
    }

    /// See [`minus`](OrdMask::minus).
    pub fn minus_from_iter(
        &self,
        others: impl IntoIterator<Item = impl AsRef<OrdMask<T>>>,
    ) -> Self {
        Self::minus(self, others.into_iter().collect::<Vec<_>>().as_slice())
    }

    /// Create a new [`OrdMask`] representing the symmetric difference of `self` and `other`.
    ///
    /// A value is included in the result if it's included in exactly one of `self` or `other`.
    ///
    /// `a.symmetric_difference(&b)` has the same result as `&a ^ &b`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::ordmask;
    ///
    /// let mask1 = ordmask![0, 10];
    /// let mask2 = ordmask![5, 20];
    /// let symmetric_difference = mask1.symmetric_difference(&mask2);
    /// assert_eq!(symmetric_difference, ordmask![0, 5, 10, 20]);
    /// ```
    pub fn symmetric_difference(&self, other: impl AsRef<Self>) -> Self {
        Self::from_suspicious_points_set(
            Self::get_suspicious_points([self, other.as_ref()]),
            |x| self.included(x) != other.as_ref().included(x),
            self.based_on_universal != other.as_ref().based_on_universal,
        )
    }

    /// Consume `self` and return its complement.
    ///
    /// A value is included in the result iff it's excluded in `self`.
    ///
    /// `mask.to_complement()` has the same result as `!mask`.
    ///
    /// For a non-consuming version, use [`.complement()`](OrdMask::complement).
    /// For an in-place version, use [`.reverse()`](OrdMask::reverse).
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::ordmask;
    ///
    /// let mask = ordmask![0, 10];
    /// let complement = mask.to_complement();
    /// assert_eq!(complement, ordmask![.., 0, 10]);
    /// ```
    pub fn to_complement(self) -> Self {
        Self {
            key_points: self.key_points,
            based_on_universal: !self.based_on_universal,
        }
    }

    /// Create a new [`OrdMask`] representing the complement of `self`.
    ///
    /// A value is included in the result iff it's excluded in `self`.
    ///
    /// `mask.complement()` has the same result as `!&mask`.
    ///
    /// For a consuming version, use [`.to_complement()`](OrdMask::to_complement).
    /// For an in-place version, use [`.reverse()`](OrdMask::reverse).
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::ordmask;
    ///
    /// let mask = ordmask![0, 10];
    /// let complement = mask.complement();
    /// assert_eq!(complement, ordmask![.., 0, 10]);
    /// ```
    pub fn complement(&self) -> Self {
        Self {
            key_points: self.key_points.clone(),
            based_on_universal: !self.based_on_universal,
        }
    }

    /// Convert `self` to its complement in place.
    ///
    /// A value is included after reversal iff it was excluded before.
    ///
    /// For a non-consuming version, use [`.complement()`](OrdMask::complement).
    /// For a consuming version, use [`.to_complement()`](OrdMask::to_complement).
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::ordmask;
    ///
    /// let mut mask = ordmask![0, 10];
    /// mask.reverse();
    /// assert_eq!(mask, ordmask![.., 0, 10]);
    /// mask.reverse();
    /// assert_eq!(mask, ordmask![0, 10]);
    /// ```
    pub fn reverse(&mut self) {
        self.based_on_universal = !self.based_on_universal;
    }
}
