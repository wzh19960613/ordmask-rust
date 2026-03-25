/// A trait to provide a non-overflowing result of subtraction.
///
/// Target type should be large enough to contain any valid result except the max valid result,
/// which should be `Self::MAX - Self::MIN`. Typically use the unsigned type as the target.
///
/// This trait must be implemented for `T` in [`OrdMask<T>`](crate::OrdMask)
/// if [`.spans().sum_size()`](crate::spans::SumSize::sum_size) is used.
///
/// The library provides implementations for all standard integer types:
/// `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
///
/// > **Note**: the type returned is [OrderedSub::Target] may be different from `Self`.
///
/// # Example
///
/// ```
/// use ordmask::{OrderedSub, WithMax, WithMin, ordmask, spans::SumSize};
///
/// // 2147483647 - (-2147483648) = 4294967295
/// assert_eq!(i32::MAX.ordered_sub(&i32::MIN), u32::MAX);
///
/// #[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
/// struct MyType(i32);
///
/// impl WithMin for MyType {
///     const MIN: Self = MyType(i32::MIN);
/// }
///
/// impl WithMax for MyType {
///     const MAX: Self = MyType(i32::MAX);
/// }
///
/// impl OrderedSub for MyType {
///     type Target = u32;
///
///     fn ordered_sub(&self, other: &Self) -> Self::Target {
///         self.0.ordered_sub(&other.0) // Same as the library does for i32
///     }
/// }
///
/// assert_eq!(ordmask![MyType(0), MyType(10)].spans().sum_size(), 10);
/// ```
pub trait OrderedSub: Ord {
    /// The type that should be returned from [OrderedSub::ordered_sub]
    type Target;
    /// The result of subtracting `other` from `self`.
    /// This method should never overflow and always return the correct value.
    /// Note that the type returned is [OrderedSub::Target] may be different from `Self`.
    fn ordered_sub(&self, other: &Self) -> Self::Target;
}

macro_rules! impl_ordered_sub_for {
    ($T: ty => $Target: ty) => {
        impl OrderedSub for $T {
            type Target = $Target;
            fn ordered_sub(&self, other: &Self) -> Self::Target {
                if self > other {
                    self.wrapping_sub(*other) as Self::Target
                } else {
                    0
                }
            }
        }
    };

    ($($T: ty), + => $Target: ty) => {
        $(impl_ordered_sub_for!($T => $Target);)+
    };
}

impl_ordered_sub_for! {i8, u8 => u8}
impl_ordered_sub_for! {i16, u16 => u16}
impl_ordered_sub_for! {i32, u32 => u32}
impl_ordered_sub_for! {i64, u64 => u64}
impl_ordered_sub_for! {i128, u128 => u128}
impl_ordered_sub_for! {isize, usize => usize}
