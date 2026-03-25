/// A trait for types that have a maximum value.
///
/// This trait must be implemented for `T` in [`OrdMask<T>`](crate::OrdMask)
/// if [`.spans()`](crate::OrdMask::spans)
/// or [`.into_spans()`](crate::OrdMask::into_spans) is used.
///
/// The library provides implementations for all standard integer types:
/// `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
///
/// # Example
///
/// ```
/// use ordmask::{WithMax, WithMin, ordmask};
///
/// #[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
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
/// assert_eq!(
///     ordmask![..].spans().collect::<Vec<_>>(),
///     vec![(MyType(i32::MIN), MyType(i32::MAX))],
/// );
///
/// ```
pub trait WithMax {
    /// The maximum value for this type.
    const MAX: Self;
}

macro_rules! impl_for {
    ($($T: ty),+ ) => { $( impl WithMax for $T { const MAX: Self = <$T>::MAX; } )+ };
}

impl_for! {u8, u16, u32, u64, u128, usize}
impl_for! {i8, i16, i32, i64, i128, isize}
