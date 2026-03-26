/// A trait for types that have a one value.
///
/// The library provides implementations for all standard integer types:
/// `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
pub trait WithOne {
    /// The one value for this type.
    const ONE: Self;
}

macro_rules! impl_for {
    ($($T: ty),+ ) => { $( impl WithOne for $T { const ONE: Self = 1; } )+ };
}

impl_for! {u8, u16, u32, u64, u128, usize}
impl_for! {i8, i16, i32, i64, i128, isize}
