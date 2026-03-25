//! Module for efficiently iterating over the included spans of an [`OrdMask`].

use std::ops::AddAssign;

use super::OrdMask;
use crate::{Countable, OrderedSub, WithMax, WithMin};

impl<T: Ord + Clone + WithMin> OrdMask<T> {
    /// Returns the number of included spans.
    ///
    /// Unlike [`.spans().count()`](Iterator::count) from the standard library,
    /// this method is **O(1)** and does not consume or iterate over the spans.
    ///
    /// See also [`.spans()`](OrdMask::spans).
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::ordmask;
    /// assert_eq!(ordmask![.., 10].spans_count(), 1);        // [MIN, 10)
    /// assert_eq!(ordmask![.., 10, 20].spans_count(), 2);    // [MIN, 10), [20, MAX]
    /// assert_eq!(ordmask![<u32>].spans_count(), 0);         // Empty
    /// assert_eq!(ordmask![<u32>..].spans_count(), 1);       // [MIN, MAX]
    /// ```
    pub const fn spans_count(&self) -> usize {
        let delta = if self.based_on_universal { 2 } else { 1 };
        (delta + self.key_points.len()) / 2
    }
}

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

/// Trait for getting the sum size of all included spans for [`OrdMask`].
pub trait SumSize<T, COUNT>
where
    T: Ord + Clone + WithMin + WithMax + OrderedSub<Target = COUNT>,
    COUNT: Countable,
{
    /// Returns the sum of the included spans.
    ///
    /// Can be used as [`.spans().sum_size()`](OrdMask::spans)
    /// or [`.into_spans().sum_size()`](OrdMask::into_spans)
    ///
    /// # Panics
    ///
    /// May panic due to overflow when called on a universal mask
    /// (because `MAX - MIN + 1` overflows). Use [`.is_universal()`](OrdMask::is_universal)
    /// to check before calling this method.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::{ordmask, spans::SumSize};
    /// // [0, 10)
    /// assert_eq!(ordmask![<u32> .., 10].spans().sum_size(), 10);
    /// // [0, 10), [20, MAX]
    /// assert_eq!(ordmask![<u32> .., 10, 20].spans().sum_size(), u32::MAX - 10 + 1);
    /// // Empty mask has size 0
    /// assert_eq!(ordmask![<u32>].spans().sum_size(), 0);
    /// // Should panic: u32::MAX + 1 is out of range
    /// // ordmask![<u32> ..].spans().sum_size();
    /// ```
    fn sum_size(&self) -> COUNT;
}

/// For [`Iter`] and [`IntoIter`].
impl<T, R, COUNT> SumSize<T, COUNT> for R
where
    T: Ord + Clone + WithMin + WithMax + OrderedSub<Target = COUNT>,
    R: Ref<T>,
    COUNT: Countable,
{
    fn sum_size(&self) -> COUNT {
        let mut sum = COUNT::ZERO;
        for i in self.mask().spans().map(|(a, b)| b.ordered_sub(&a)) {
            sum = sum + i;
        }
        if self.mask().is_max_value_included() {
            sum = sum + COUNT::ONE;
        }
        sum
    }
}

impl<T: Ord + Clone + WithMin + WithMax> OrdMask<T> {
    /// Returns an iterator over the included spans.
    ///
    /// Each span is a tuple `(start, end)` representing a half-open interval `[start, end)`.
    ///
    /// See also [`.into_spans()`](OrdMask::into_spans), [`.spans_count()`](OrdMask::spans_count).
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
    /// See also [`.spans()`](OrdMask::spans), [`.spans_count()`](OrdMask::spans_count).
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
