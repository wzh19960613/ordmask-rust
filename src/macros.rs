/// Create an [`OrdMask`](crate::OrdMask) from key points.
///
/// # Panics
///
/// Panics if key points are not in non-decreasing order.
///
/// # Examples
///
/// ```
/// use ordmask::{OrdMask, ordmask};
///
/// let mask = ordmask![<i32>]; // or: let mask: OrdMask<i32> = ordmask![];
/// assert_eq!(mask, OrdMask::<i32>::empty());
///
/// let mask = ordmask![<u64>..]; // or: let mask: OrdMask<u64> = ordmask![..];
/// assert_eq!(mask, OrdMask::<u64>::universal());
///
/// let mask = ordmask![0, 10, 20]; // can also use ordmask![<i32> 0, 10, 20]
/// assert_eq!(mask, OrdMask::from(vec![0, 10, 20]));
///
/// let mask = ordmask![.., 0, 10, 20]; // can also use ordmask![<i32> .., 0, 10, 20]
/// assert_eq!(mask, OrdMask::from_complement(vec![0, 10, 20]));
/// ```
#[macro_export]
macro_rules! ordmask {
    ($(<$T: ty>)?) => {
        ordmask::OrdMask$(::<$T>)?::empty()
    };
    ($(<$T: ty>)? ..) => {
        ordmask::OrdMask$(::<$T>)?::universal()
    };
    ($(<$T: ty>)? .., $($key_points:expr),+ $(,)?) => {
        ordmask::OrdMask$(::<$T>)?::from_complement(vec![$($key_points),+])
    };
    (<$T: ty> $($key_points:expr),+ $(,)?) => {
        ordmask::OrdMask::<$T>::from(vec![$($key_points),+])
    };
    ($($key_points:expr),+ $(,)?) => {
        ordmask::OrdMask::from(vec![$($key_points),+])
    };
}

/// Create an [`OrdMask`](crate::OrdMask) from key points without validation.
///
/// Usage is similar to [`ordmask!`], but
/// without the non-decreasing check and [`OrdMask::simplify`](crate::OrdMask::simplify).
///
/// # Safety
///
/// Key points must be non-decreasing.
#[macro_export]
macro_rules! ordmask_uncheck {
    ($($T: ty)?) => {
        ordmask::OrdMask$(::<$T>)?::empty()
    };
    ($($T: ty;)? ..) => {
        ordmask::OrdMask$(::<$T>)?::universal()
    };
    ($($T: ty;)? .., $($key_points:expr),+ $(,)?) => {
        ordmask::OrdMask$(::<$T>)?::with_checked(vec![$($key_points),+], true)
    };
    ($T: ty; $($key_points:expr),+ $(,)?) => {
        ordmask::OrdMask::<$T>::with_checked(vec![$($key_points),+], false)
    };
    ($($key_points:expr),+ $(,)?) => {
        ordmask::OrdMask::with_checked(vec![$($key_points),+], false)
    };
}
