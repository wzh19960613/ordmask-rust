/// A trait for getting the minimum value of a type.
///
/// Has been implemented for all primitive numeric types.
///
/// `impl MinValue for T {...}` if you need a universal or negative `OrdMask<T>`.
pub trait MinValue {
    /// Get the minimum value of the type.
    fn min_value() -> Self;
}

macro_rules! impl_min {
    ($($t:ty),*) => ($(
        impl MinValue for $t {
            fn min_value() -> Self {
                <$t>::MIN
            }
        }
    )*)
}

impl_min!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
