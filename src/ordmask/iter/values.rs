//! Value iterators for [`OrdMask`](crate::OrdMask).

use std::ops::{Add, AddAssign};

use crate::{OrdMask, WithMax, WithMin, WithOne};

/// An iterator over the included values of an [`OrdMask`].
///
/// This iterator is created by [`.values()`](OrdMask::values).
pub struct Iter<'a, T: Ord + Clone + WithMin + WithMax + Add<Output = T> + WithOne> {
    mask: &'a OrdMask<T>,
    span_index: usize,
    current_value: Option<T>,
    current_end: Option<T>,
}

/// An owning iterator over the included values of an [`OrdMask`].
///
/// This iterator is created by [`.into_values()`](OrdMask::into_values).
pub struct IntoIter<T: Ord + Clone + WithMin + WithMax + Add<Output = T> + WithOne> {
    mask: OrdMask<T>,
    span_index: usize,
    current_value: Option<T>,
    current_end: Option<T>,
}

trait IterRef<T: Ord + Clone + WithMin + WithMax + Add<Output = T> + WithOne> {
    fn mask(&self) -> &OrdMask<T>;
    fn span_index(&mut self) -> &mut usize;
    fn current_value(&mut self) -> &mut Option<T>;
    fn current_end(&mut self) -> &mut Option<T>;
}

impl<'a, T> IterRef<T> for Iter<'a, T>
where
    T: Ord + Clone + WithMin + WithMax + Add<Output = T> + WithOne,
{
    fn mask(&self) -> &OrdMask<T> {
        self.mask
    }

    fn span_index(&mut self) -> &mut usize {
        &mut self.span_index
    }

    fn current_value(&mut self) -> &mut Option<T> {
        &mut self.current_value
    }

    fn current_end(&mut self) -> &mut Option<T> {
        &mut self.current_end
    }
}

impl<T: Ord + Clone + WithMin + WithMax + Add<Output = T> + WithOne> IterRef<T> for IntoIter<T> {
    fn mask(&self) -> &OrdMask<T> {
        &self.mask
    }

    fn span_index(&mut self) -> &mut usize {
        &mut self.span_index
    }

    fn current_value(&mut self) -> &mut Option<T> {
        &mut self.current_value
    }

    fn current_end(&mut self) -> &mut Option<T> {
        &mut self.current_end
    }
}

trait IterImpl<T: Ord + Clone + WithMin + WithMax + Add<Output = T> + WithOne> {
    fn pop_key_point(&mut self) -> Option<T>;
    fn value_iter_next(&mut self) -> Option<T>;
}

impl<T, R> IterImpl<T> for R
where
    T: Ord + Clone + WithMin + WithMax + Add<Output = T> + WithOne,
    R: IterRef<T>,
{
    fn pop_key_point(&mut self) -> Option<T> {
        let idx = *self.span_index();
        let points = &self.mask().key_points;
        if idx >= points.len() {
            return None;
        }
        let item = points[idx].clone();
        self.span_index().add_assign(1);
        Some(item)
    }

    fn value_iter_next(&mut self) -> Option<T> {
        if self.current_value().is_some() {
            let current = self.current_value().clone().unwrap();
            let end = self.current_end().clone().unwrap();
            if current < end {
                let result = current.clone();
                if current < T::MAX {
                    *self.current_value() = Some(current + T::ONE);
                } else {
                    *self.current_value() = None;
                    *self.current_end() = None;
                }
                return Some(result);
            }

            if end == T::MAX && self.mask().is_max_value_included() && current == T::MAX {
                *self.current_value() = None;
                *self.current_end() = None;
                return Some(T::MAX);
            }

            *self.current_value() = None;
            *self.current_end() = None;
        }

        let start = if *self.span_index() == 0 && self.mask().based_on_universal {
            if self.mask().key_points.is_empty() {
                self.span_index().add_assign(1);
                T::MIN
            } else {
                T::MIN
            }
        } else {
            self.pop_key_point()?
        };

        let end = self.pop_key_point().unwrap_or(T::MAX);
        if start >= end {
            return self.value_iter_next();
        }

        let result = start.clone();
        if start < T::MAX {
            *self.current_value() = Some(start + T::ONE);
            *self.current_end() = Some(end);
        } else {
            *self.current_value() = None;
            *self.current_end() = None;
        }
        Some(result)
    }
}

impl<'a, T: Ord + Clone + WithMin + WithMax + Add<Output = T> + WithOne> Iterator for Iter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.value_iter_next()
    }
}

impl<T: Ord + Clone + WithMin + WithMax + Add<Output = T> + WithOne> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.value_iter_next()
    }
}

impl<T: Ord + Clone + WithMin + WithMax + Add<Output = T> + WithOne> OrdMask<T> {
    /// Returns an iterator over the included values.
    ///
    /// Unlike [`.spans()`](OrdMask::spans) which returns intervals,
    /// this method returns each individual value in the mask.
    ///
    /// See also [`.into_values()`](OrdMask::into_values).
    ///
    /// > Use [`.values_count()`](OrdMask::values_count)
    /// > instead of [`.values().count()`](std::iter::Iterator::count)
    /// > to be more efficient when you only need the number of values.
    ///
    /// **Warning**: For large masks (e.g., universal mask), this can iterate
    /// over a huge number of values. Use with caution.
    ///
    /// # Examples
    /// ```
    /// use ordmask::ordmask;
    ///
    /// // Empty mask has no values
    /// assert_eq!(ordmask![<i32>].values().collect::<Vec<_>>(), vec![]);
    ///
    /// // Single value range [1, 2)
    /// assert_eq!(ordmask![1, 2].values().collect::<Vec<_>>(), vec![1]);
    ///
    /// // Multiple values [1, 4)
    /// assert_eq!(ordmask![1, 4].values().collect::<Vec<_>>(), vec![1, 2, 3]);
    ///
    /// // Multiple spans: [1, 3) and [5, 7)
    /// assert_eq!(
    ///     ordmask![1, 3, 5, 7].values().collect::<Vec<_>>(),
    ///     vec![1, 2, 5, 6]
    /// );
    /// ```
    pub fn values(&self) -> Iter<'_, T> {
        Iter {
            mask: self,
            span_index: 0,
            current_value: None,
            current_end: None,
        }
    }

    /// Consumes the mask and returns an iterator over the included values.
    ///
    /// Unlike [`.into_spans()`](OrdMask::into_spans) which returns intervals,
    /// this method returns each individual value in the mask.
    ///
    /// See also [`.values()`](OrdMask::values).
    ///
    /// > Use [`.values_count()`](OrdMask::values_count)
    /// > instead of [`.values().count()`](std::iter::Iterator::count)
    /// > to be more efficient when you only need the number of values.
    ///
    /// **Warning**: For large masks (e.g., universal mask), this can iterate
    /// over a huge number of values. Use with caution.
    ///
    /// # Examples
    /// ```
    /// use ordmask::ordmask;
    ///
    /// // Multiple values [1, 4)
    /// assert_eq!(ordmask![1, 4].into_values().collect::<Vec<_>>(), vec![1, 2, 3]);
    /// ```
    pub fn into_values(self) -> IntoIter<T> {
        IntoIter {
            mask: self,
            span_index: 0,
            current_value: None,
            current_end: None,
        }
    }
}
