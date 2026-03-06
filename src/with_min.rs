/// A trait for types that have a minimum value.
///
/// The library provides implementations for all standard integer types:
/// `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
///
/// This trait must be implemented for `T` in `OrdMask<T>`.
///
/// # Example
///
/// ```
/// use ordmask::{WithMin, ordmask};
///
/// assert_eq!(i32::MIN, <i32 as WithMin>::MIN);
///
/// #[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
/// struct MyType(i32);
///
/// impl WithMin for MyType {
///     const MIN: Self = MyType(i32::MIN);
/// }
///
/// assert!(ordmask![..].included(&MyType(1)));
///
/// ```
pub trait WithMin {
    /// The minimum value for this type.
    const MIN: Self;
}

macro_rules! impl_for {
    ($($T: ty),+ ) => { $( impl WithMin for $T { const MIN: Self = <$T>::MIN; } )+ };
}

impl_for! {u8, u16, u32, u64, u128, usize}
impl_for! {i8, i16, i32, i64, i128, isize}
