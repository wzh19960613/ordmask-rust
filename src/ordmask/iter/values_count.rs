use std::ops::Add;

use crate::{OrdMask, OrderedSub, WithMax, WithMin, WithOne, WithZero};

/// A lazily-computed result of the count of all included values.
///
/// This struct is returned by [`.values_count()`](OrdMask::values_count).
///
/// # Lazy Evaluation
///
/// `ValuesCount` supports direct comparison with `COUNT` values without computing
/// the full count. When you compare a `ValuesCount` with a value (e.g., `result == 10`,
/// `result < 100`), the computation stops as soon as the result can be determined.
///
/// This is especially useful when:
/// - You only need to check if the count exceeds a threshold
/// - The mask may cause overflow when fully computed
/// - You want early termination for performance
///
/// To get the actual count value, use [`.get()`](ValuesCount::get).
///
/// # Examples
///
/// ```
/// use ordmask::ordmask;
///
/// let mask = ordmask![<u32> .., 10];
/// let result = mask.values_count();
///
/// // Direct comparison (lazy, stops early when possible)
/// assert!(result == 10);
/// assert!(result < 20);
/// assert!(result > 5);
///
/// // Get the actual value (eager, computes full count)
/// assert_eq!(result.get(), 10);
/// ```
pub struct ValuesCount<'a, T, COUNT>
where
    T: Ord + Clone + WithMin + WithMax + OrderedSub<Target = COUNT>,
    COUNT: WithZero + WithOne + Add<COUNT, Output = COUNT>,
{
    mask: &'a OrdMask<T>,
}

impl<'a, T, COUNT> ValuesCount<'a, T, COUNT>
where
    T: Ord + Clone + WithMin + WithMax + OrderedSub<Target = COUNT>,
    COUNT: WithZero + WithOne + Add<COUNT, Output = COUNT>,
{
    /// Returns the count of all included values.
    ///
    /// Comparing `ValuesCount` with a `COUNT` value is more efficient
    /// than calling this to compute the full count, if you only need to compare
    /// the count against a known value (e.g., `result == 10`, `result < 100`).
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
    /// use ordmask::ordmask;
    /// // [0, 10)
    /// assert_eq!(ordmask![<u32> .., 10].values_count().get(), 10);
    /// // [0, 10), [20, MAX]
    /// assert_eq!(ordmask![<u32> .., 10, 20].values_count().get(), u32::MAX - 10 + 1);
    /// // Empty mask has count 0
    /// assert_eq!(ordmask![<u32>].values_count().get(), 0);
    /// // Should panic: u32::MAX + 1 is out of range
    /// // ordmask![<u32> ..].values_count().get();
    /// ```
    pub fn get(&self) -> COUNT {
        let mut sum = COUNT::ZERO;
        for i in self.mask.spans().map(|(a, b)| b.ordered_sub(&a)) {
            sum = sum + i;
        }
        if self.mask.is_max_value_included() {
            sum = sum + COUNT::ONE;
        }
        sum
    }

    #[inline(always)]
    fn compare_with<StopCmp, Cmp>(&self, value: &COUNT, stop_cmp: StopCmp, cmp: Cmp) -> bool
    where
        StopCmp: Fn(&COUNT, &COUNT) -> bool,
        Cmp: Fn(&COUNT, &COUNT) -> bool,
    {
        let mut sum = COUNT::ZERO;
        for i in self.mask.spans().map(|(a, b)| b.ordered_sub(&a)) {
            sum = sum + i;
            if stop_cmp(&sum, value) {
                return false;
            }
        }
        if self.mask.is_max_value_included() {
            sum = sum + COUNT::ONE;
        }
        cmp(&sum, value)
    }
}

impl<'a, T, COUNT> PartialEq<COUNT> for ValuesCount<'a, T, COUNT>
where
    T: Ord + Clone + WithMin + WithMax + OrderedSub<Target = COUNT>,
    COUNT: WithZero + WithOne + Add<COUNT, Output = COUNT> + PartialOrd,
{
    fn eq(&self, other: &COUNT) -> bool {
        self.compare_with(other, PartialOrd::gt, PartialEq::eq)
    }
}

impl<'a, T, COUNT> PartialOrd<COUNT> for ValuesCount<'a, T, COUNT>
where
    T: Ord + Clone + WithMin + WithMax + OrderedSub<Target = COUNT>,
    COUNT: WithZero + WithOne + Add<COUNT, Output = COUNT> + PartialOrd,
{
    fn partial_cmp(&self, other: &COUNT) -> Option<std::cmp::Ordering> {
        let sum = self.get();
        sum.partial_cmp(other)
    }

    fn lt(&self, other: &COUNT) -> bool {
        self.compare_with(other, PartialOrd::ge, PartialOrd::lt)
    }

    fn le(&self, other: &COUNT) -> bool {
        self.compare_with(other, PartialOrd::gt, PartialOrd::le)
    }

    fn gt(&self, other: &COUNT) -> bool {
        !self.le(other)
    }

    fn ge(&self, other: &COUNT) -> bool {
        !self.lt(other)
    }
}

impl<T, COUNT> OrdMask<T>
where
    T: Ord + Clone + WithMin + WithMax + OrderedSub<Target = COUNT>,
    COUNT: WithZero + WithOne + Add<COUNT, Output = COUNT>,
{
    /// Returns a lazily-computed count of all included values.
    ///
    /// The returned [`ValuesCount`] can be compared directly with `COUNT` values
    /// without computing the full count (lazy evaluation). To get the actual value,
    /// call [`.get()`](ValuesCount::get).
    ///
    /// # Panics
    ///
    /// Calling `.get()` may panic due to overflow when called on a universal mask
    /// (because `MAX - MIN + 1` overflows). Use [`.is_universal()`](OrdMask::is_universal)
    /// to check before calling `.get()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use ordmask::ordmask;
    /// // [0, 10)
    /// let mask = ordmask![<u32> .., 10];
    /// let result = mask.values_count();
    /// // Lazy comparison - efficient!
    /// assert!(result == 10);
    /// assert!(result < 20);
    /// // Eager computation
    /// assert_eq!(result.get(), 10);
    /// ```
    pub fn values_count(&self) -> ValuesCount<'_, T, COUNT> {
        ValuesCount { mask: self }
    }
}
