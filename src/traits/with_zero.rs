/// A trait for types that have a zero value.
///
/// The library provides implementations for all standard integer types:
/// `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
pub trait WithZero {
    /// The zero value for this type.
    const ZERO: Self;
}

macro_rules! impl_for {
    ($($T: ty),+ ) => { $( impl WithZero for $T { const ZERO: Self = 0; } )+ };
}

impl_for! {u8, u16, u32, u64, u128, usize}
impl_for! {i8, i16, i32, i64, i128, isize}
