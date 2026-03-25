/// A trait for types that has a `ZERO` and `ONE` constant, and can be added to itself.
///
/// The library provides implementations for all standard integer types:
/// `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
pub trait Countable: std::ops::Add<Self, Output = Self>
where
    Self: Sized,
{
    /// The `0` value for this type.
    const ZERO: Self;
    /// The `1` value for this type.
    const ONE: Self;
}

macro_rules! impl_for {
    ($($T: ty),+ ) => { $( impl Countable for $T {
        const ZERO: Self = 0;
        const ONE: Self = 1;
    } )+ };
}

impl_for! {u8, u16, u32, u64, u128, usize}
impl_for! {i8, i16, i32, i64, i128, isize}
