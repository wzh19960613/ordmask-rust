use super::OrdMask;

impl<T: Ord + Clone> OrdMask<T> {
    /// Create a new OrdMask representing the union of the `masks`.
    ///
    /// Values included in the union must be included in at least one of the `masks`.
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
    pub fn union(masks: &[&OrdMask<T>]) -> Self {
        Self::from_key_points_set(
            Self::get_key_points_set(masks),
            |x| masks.iter().any(|item| item.included(x)),
            masks.iter().any(|item| item.reversed),
        )
    }

    /// Create a new OrdMask representing the intersection of the `masks`.
    ///
    /// Values included in the intersection must be included in all of the `masks`.
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
    pub fn intersection(masks: &[&OrdMask<T>]) -> Self {
        Self::from_key_points_set(
            Self::get_key_points_set(masks),
            |x| masks.iter().all(|item| item.included(x)),
            masks.iter().all(|item| item.reversed),
        )
    }

    /// Create a new OrdMask representing the difference of the `self` and `others`.
    ///
    /// Values included in the difference must be included in `self`
    /// and excluded in all of the `others`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::ordmask;
    ///
    /// let mask1 = ordmask![0, 30];
    /// let mask2 = ordmask![5, 10];
    /// let mask3 = ordmask![6, 20];
    /// let complement = mask1.minus(&[&mask2, &mask3]);
    /// assert_eq!(complement, ordmask![0, 5, 20, 30]);
    /// ```
    pub fn minus(&self, others: &[&OrdMask<T>]) -> Self {
        Self::from_key_points_set(
            Self::get_key_points_set(&[&[self], others].concat()),
            |x| self.included(x) && others.iter().all(|item| item.excluded(x)),
            self.reversed && !others.iter().any(|item| item.reversed),
        )
    }

    /// Create a new OrdMask representing the symmetric difference of the `self` and `other`.
    ///
    /// Values included in the symmetric difference
    /// must be included in one of the `self` or `other`, but not both.
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
    pub fn symmetric_difference(&self, other: &Self) -> Self {
        Self::from_key_points_set(
            Self::get_key_points_set(&[self, other]),
            |x| self.included(x) != other.included(x),
            self.reversed ^ other.reversed,
        )
    }

    /// Consume the `self` and return a new OrdMask that represents the complement of the `self`.
    ///
    /// Values included in the complement must be excluded in the `self`, and vice versa.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::ordmask;
    ///
    /// let mask = ordmask![0, 10];
    /// let complement = mask.complement();
    /// assert_eq!(complement, ordmask![_, 0, 10]);
    /// ```
    pub fn complement(self) -> Self {
        Self {
            key_points: self.key_points,
            reversed: !self.reversed,
        }
    }

    /// Create a new OrdMask that represents the complement of the `self`.
    ///
    /// Values included in the complement must be excluded in the `self`, and vice versa.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::ordmask;
    ///
    /// let mask = ordmask![0, 10];
    /// let complement = mask.new_complement();
    /// assert_eq!(complement, ordmask![_, 0, 10]);
    /// ```
    pub fn new_complement(&self) -> Self {
        Self {
            key_points: self.key_points.clone(),
            reversed: !self.reversed,
        }
    }

    /// Convert the `self` to its complement.
    ///
    /// Values included in the complement must be excluded in the `self`, and vice versa.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::ordmask;
    ///
    /// let mut mask = ordmask![0, 10];
    /// mask.reverse();
    /// assert_eq!(mask, ordmask![_, 0, 10]);
    /// mask.reverse();
    /// assert_eq!(mask, ordmask![0, 10]);
    /// ```
    pub fn reverse(&mut self) {
        self.reversed = !self.reversed;
    }
}
