//! Span iterators for [`OrdMask`].

use std::ops::AddAssign;

use crate::{OrdMask, WithMax, WithMin};

/// An iterator over the included spans of an [`OrdMask`].
///
/// Each span is returned as a tuple `(start, end)` representing a half-open interval `[start, end)`.
///
/// This iterator is created by [`.spans()`](OrdMask::spans).
pub struct Iter<'a, T: Ord + Clone + WithMin> {
    mask: &'a OrdMask<T>,
    index: usize,
}

/// An owning iterator over the included spans of an [`OrdMask`].
///
/// Each span is returned as a tuple `(start, end)` representing a half-open interval `[start, end)`.
///
/// This iterator is created by [`.into_spans()`](OrdMask::into_spans).
pub struct IntoIter<T: Ord + Clone + WithMin> {
    mask: OrdMask<T>,
    index: usize,
}

trait Ref<T: Ord + Clone + WithMin + WithMax> {
    fn mask(&self) -> &OrdMask<T>;
    fn idx(&mut self) -> &mut usize;
}

trait SpansImpl<T: Ord + Clone + WithMin + WithMax> {
    fn pop_item(&mut self) -> Option<T>;
    fn iter_next(&mut self) -> Option<(T, T)>;
}

impl<'a, T: Ord + Clone + WithMin + WithMax> Ref<T> for Iter<'a, T> {
    fn mask(&self) -> &OrdMask<T> {
        self.mask
    }

    fn idx(&mut self) -> &mut usize {
        &mut self.index
    }
}

impl<T: Ord + Clone + WithMin + WithMax> Ref<T> for IntoIter<T> {
    fn mask(&self) -> &OrdMask<T> {
        &self.mask
    }

    fn idx(&mut self) -> &mut usize {
        &mut self.index
    }
}

impl<T, R> SpansImpl<T> for R
where
    T: Ord + Clone + WithMin + WithMax,
    R: Ref<T>,
{
    fn pop_item(&mut self) -> Option<T> {
        let idx = *self.idx();
        let points = &self.mask().key_points;
        if idx >= points.len() {
            return None;
        }
        let item = points[idx].clone();
        self.idx().add_assign(1);
        Some(item)
    }

    fn iter_next(&mut self) -> Option<(T, T)> {
        let start = if *self.idx() == 0 && self.mask().based_on_universal {
            if self.mask().key_points.is_empty() {
                self.idx().add_assign(1);
            }
            T::MIN
        } else {
            self.pop_item()?
        };
        Some((start, self.pop_item().unwrap_or(T::MAX)))
    }
}

impl<'a, T: Ord + Clone + WithMin + WithMax> Iterator for Iter<'a, T> {
    type Item = (T, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter_next()
    }
}

impl<T: Ord + Clone + WithMin + WithMax> Iterator for IntoIter<T> {
    type Item = (T, T);
    fn next(&mut self) -> Option<Self::Item> {
        self.iter_next()
    }
}

impl<T: Ord + Clone + WithMin + WithMax> OrdMask<T> {
    /// Returns an iterator over the included spans.
    ///
    /// Each span is a tuple `(start, end)` representing a half-open interval `[start, end)`.
    ///
    /// See also [`.into_spans()`](OrdMask::into_spans)
    ///
    /// > Use [`.spans_count()`](OrdMask::spans_count)
    /// > instead of [`.spans().count()`](std::iter::Iterator::count)
    /// > to be more efficient when you only need the number of spans.
    ///
    /// **Note**: Since spans are half-open intervals `[start, end)`, whether `MAX` is included
    /// can be confusing. Use [`.is_max_value_included()`](OrdMask::is_max_value_included)
    /// to check if the maximum value is in the mask.
    ///
    /// # Examples
    /// ```
    /// use ordmask::ordmask;
    ///
    /// // Empty mask has no spans
    /// assert_eq!(ordmask![<i32>].spans().collect::<Vec<_>>(), vec![]);
    ///
    /// // Universal mask has one span [MIN, MAX]
    /// assert_eq!(
    ///     ordmask![..].spans().collect::<Vec<_>>(),
    ///     vec![(i32::MIN, i32::MAX)]
    /// );
    ///
    /// // Single span [1, 2)
    /// assert_eq!(ordmask![1, 2].spans().collect::<Vec<_>>(), vec![(1, 2)]);
    ///
    /// // Multiple spans: [MIN, 1) and [2, MAX]
    /// assert_eq!(
    ///     ordmask![.., 1, 2].spans().collect::<Vec<_>>(),
    ///     vec![(i32::MIN, 1), (2, i32::MAX)]
    /// );
    /// ```
    pub fn spans(&self) -> Iter<'_, T> {
        Iter {
            mask: self,
            index: 0,
        }
    }

    /// Consumes the mask and returns an iterator over the included spans.
    ///
    /// Each span is a tuple `(start, end)` representing a half-open interval `[start, end)`.
    ///
    /// See also [`.spans()`](OrdMask::spans).
    ///
    /// > Use [`.spans_count()`](OrdMask::spans_count)
    /// > instead of [`.spans().count()`](std::iter::Iterator::count)
    /// > to be more efficient when you only need the number of spans.
    ///
    /// **Note**: Since spans are half-open intervals `[start, end)`, whether `MAX` is included
    /// can be confusing. Use [`.is_max_value_included()`](OrdMask::is_max_value_included)
    /// to check if the maximum value is in the mask.
    ///
    /// # Examples
    /// ```
    /// use ordmask::ordmask;
    ///
    /// assert_eq!(
    ///     ordmask![.., 1, 2_i32].into_spans().collect::<Vec<_>>(),
    ///     vec![(i32::MIN, 1), (2, i32::MAX)]
    /// );
    /// ```
    pub fn into_spans(self) -> IntoIter<T> {
        IntoIter {
            mask: self,
            index: 0,
        }
    }
}
