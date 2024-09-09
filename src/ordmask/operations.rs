use super::OrdMask;
use crate::MinValue;

impl<T: Ord + Clone> OrdMask<T> {
    /// Create a new OrdMask representing the union of the `masks`.
    ///
    /// Values included in the union must be included in at least one of the `masks`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::OrdMask;
    ///
    /// let mask1 = OrdMask::from(vec![0, 6]);
    /// let mask2 = OrdMask::from(vec![5, 10]);
    /// let mask3 = OrdMask::from(vec![20, 30]);
    /// let union = OrdMask::union(&[&mask1, &mask2, &mask3]);
    /// assert_eq!(union, OrdMask::from(vec![0, 10, 20, 30]));
    /// ```
    pub fn union(masks: &[&OrdMask<T>]) -> Self {
        Self::new(Self::get_key_points(masks), |x| {
            masks.iter().any(|item| item.included(x))
        })
    }

    /// Create a new OrdMask representing the intersection of the `masks`.
    ///
    /// Values included in the intersection must be included in all of the `masks`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::OrdMask;
    ///
    /// let mask1 = OrdMask::from(vec![0, 6]);
    /// let mask2 = OrdMask::from(vec![5, 10]);
    /// let mask3 = OrdMask::from(vec![-10, 30]);
    /// let intersection = OrdMask::intersection(&[&mask1, &mask2, &mask3]);
    /// assert_eq!(intersection, OrdMask::from(vec![5, 6]));
    /// ```
    pub fn intersection(masks: &[&OrdMask<T>]) -> Self {
        Self::new(Self::get_key_points(masks), |x| {
            masks.iter().all(|item| item.included(x))
        })
    }

    /// Create a new OrdMask representing the difference of the `self` and `others`.
    ///
    /// Values included in the difference must be included in `self`
    /// and excluded in all of the `others`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::OrdMask;
    ///
    /// let mask1 = OrdMask::from(vec![0, 30]);
    /// let mask2 = OrdMask::from(vec![5, 10]);
    /// let mask3 = OrdMask::from(vec![6, 20]);
    /// let difference = mask1.difference(&[&mask2, &mask3]);
    /// assert_eq!(difference, OrdMask::from(vec![0, 5, 20, 30]));
    /// ```
    pub fn difference(&self, others: &[&OrdMask<T>]) -> Self {
        Self::new(Self::get_key_points(&[&[self], others].concat()), |x| {
            self.included(x) && others.iter().all(|item| item.excluded(x))
        })
    }

    /// Create a new OrdMask representing the symmetric difference of the `self` and `other`.
    ///
    /// Values included in the symmetric difference
    /// must be included in one of the `self` or `other`, but not both.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::OrdMask;
    ///
    /// let mask1 = OrdMask::from(vec![0, 10]);
    /// let mask2 = OrdMask::from(vec![5, 20]);
    /// let symmetric_difference = mask1.symmetric_difference(&mask2);
    /// assert_eq!(symmetric_difference, OrdMask::from(vec![0, 5, 10, 20]));
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        Self::new(Self::get_key_points(&[self, other]), |x| {
            self.included(x) != other.included(x)
        })
    }
}

impl<T: Ord + Clone + MinValue> OrdMask<T> {
    /// Create a new OrdMask that represents the complement of the `self`.
    ///
    /// Values included in the complement must be excluded in the `self`, and vice versa.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::OrdMask;
    ///
    /// let mask = OrdMask::from(vec![0, 10]);
    /// let complement = mask.complement();
    /// assert_eq!(complement, OrdMask::from(vec![i32::min_value(), 0, 10]));
    /// assert_eq!(complement.complement(), OrdMask::from(vec![0, 10]));
    /// ```
    pub fn complement(&self) -> Self {
        match self.is_include_min_value() {
            true => Self(self.0[1..].to_vec()),
            false => Self([&[T::min_value()][..], &self.0].concat().to_vec()),
        }
    }
}
