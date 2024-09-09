use crate::MinValue;

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
/// use ordmask::OrdMask;
///
/// fn should_include(mask: &OrdMask<i32>, x: i32) {
///     assert!(mask.included(&x), "should include {}", x);
/// }
///
/// fn should_exclude(mask: &OrdMask<i32>, x: i32) {
///     assert!(mask.excluded(&x), "should exclude {}", x);
/// }
///
/// let mask = OrdMask::empty();
/// assert!(mask.is_empty());
/// (-10..10).for_each(|x| should_exclude(&mask, x));
///
/// let mask = OrdMask::from(vec![0]);
/// (-10..0).for_each(|x| should_exclude(&mask, x));
/// (0..10).for_each(|x| should_include(&mask, x));
///
/// let mask = OrdMask::from(vec![0, 10]);
/// (-10..0).for_each(|x| should_exclude(&mask, x));
/// (0..10).for_each(|x| should_include(&mask, x));
/// (10..20).for_each(|x| should_exclude(&mask, x));
///
/// let mask = OrdMask::from(vec![0, 10, 20]);
/// (-10..0).for_each(|x| should_exclude(&mask, x));
/// (0..10).for_each(|x| should_include(&mask, x));
/// (10..20).for_each(|x| should_exclude(&mask, x));
/// (20..30).for_each(|x| should_include(&mask, x));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrdMask<T: Ord + Clone>(Vec<T>);

impl<T: Ord + Clone> OrdMask<T> {
    /// Get the length of the `OrdMask`.
    ///
    /// It means how many values are used to represent this mask, not the number of valid values.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the `OrdMask` is empty.
    ///
    /// An empty `OrdMask` means no value is included.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Check if the `OrdMask` is valid.
    ///
    /// There is no need to test `is_valid()` if you never use unsafe methods.
    pub fn is_valid(&self) -> bool {
        convert::get_first_falling_index(&self.0) == 0
    }

    /// Check if a value is included in this mask.
    pub fn included(&self, value: &T) -> bool {
        self.0.partition_point(|x| x <= value) % 2 == 1
    }

    /// Check if a value is excluded in this mask.
    pub fn excluded(&self, value: &T) -> bool {
        !self.included(value)
    }

    pub fn is_include_max_value(&self) -> bool {
        self.0.len() % 2 == 1
    }

    /// Check if the `OrdMask` is simplified.
    ///
    /// An simplified `OrdMask` means there are no duplicate values.
    ///
    /// There is no need to test `is_simplified()` if you never use unsafe methods,  
    /// otherwise you should directly use `simplify()` without checking `is_simplified()`.
    pub fn is_simplified(&self) -> bool {
        for i in 1..self.len() {
            if self.0[i] == self.0[i - 1] {
                return false;
            }
        }
        true
    }

    /// Remove meaningless values in the `OrdMask`.
    ///
    /// Other safe methods will ensure the `OrdMask` is simplified.  
    /// Therefore, there is no need to call it unless you are using unsafe methods.
    ///
    /// # Examples
    /// ```
    /// use ordmask::OrdMask;
    ///
    /// let mut mask = OrdMask::from(vec![0, 0, 1, 1]);
    /// assert_eq!(mask, OrdMask::from(vec![]));
    ///
    /// let mut mask = OrdMask::from(vec![0, 2, 2, 2, 3, 3, 4]);
    /// assert_eq!(mask, OrdMask::from(vec![0, 2, 4]));
    ///
    /// let mut mask = unsafe { OrdMask::with_unchecked(vec![0, 2, 2, 2, 4, 4, 4, 6, 6, 8, 8]) };
    /// mask.simplify();
    /// assert_eq!(mask, OrdMask::from(vec![0, 2, 4]));
    /// ```
    pub fn simplify(&mut self) {
        let len = self.len();
        if len < 2 {
            return;
        }

        let mut write_index = 0;
        let mut read_index = 0;
        while read_index < len {
            let start = read_index;
            while read_index < len && self.0[read_index] == self.0[start] {
                read_index += 1;
            }
            if (read_index - start) % 2 == 1 {
                self.0[write_index] = self.0[start].clone();
                write_index += 1;
            }
        }

        self.0.truncate(write_index);
    }

    /// Get the key points of `masks`. The key points are all values for representing a `OrdMask`.
    pub fn get_key_points(masks: &[&OrdMask<T>]) -> std::collections::BTreeSet<T> {
        let mut result = std::collections::BTreeSet::new();
        for item in masks {
            result.extend(item.0.iter().cloned());
        }
        result
    }
}

impl<T: Ord + Clone + MinValue> OrdMask<T> {
    /// Check if the mask includes the minimum value.
    pub fn is_include_min_value(&self) -> bool {
        self.0[0] == T::min_value()
    }

    /// Check if the mask is universal.
    ///
    /// An universal mask includes all values.
    pub fn is_universal(&self) -> bool {
        self.0.len() == 1 && self.is_include_min_value()
    }
}

#[cfg(test)]
mod tests {
    use super::OrdMask;

    #[test]
    fn test_simplify() {
        let test_cases = vec![
            (vec![], vec![]),
            (vec![0, 0, 1, 1], vec![]),
            (vec![0, 2, 2, 4], vec![0, 4]),
            (vec![0, 2, 2, 2, 4, 4, 4, 6, 6, 8, 8], vec![0, 2, 4]),
            (vec![1, 1, 1, 1, 2, 2, 3, 3, 3, 3], vec![]),
            (
                vec![1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 5, 5, 6, 6],
                vec![1, 2, 3],
            ),
            (vec![1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 4], vec![2]),
            (vec![1, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5, 5], vec![1, 3]),
            (vec![1, 1, 1, 2, 2, 3, 3, 3, 4, 4, 5, 5, 5], vec![1, 3, 5]),
            (vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![]),
            (
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            ),
        ];

        for (input, expected) in test_cases {
            let mask = OrdMask::from(input.clone());
            assert_eq!(
                mask,
                OrdMask::from(expected.clone()),
                "Test failed for input: {:?}, expected: {:?}, got: {:?}",
                input,
                expected,
                mask
            );
        }
    }
}
