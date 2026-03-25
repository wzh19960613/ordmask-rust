use thiserror::Error;

/// Error for [`OrdMask`](crate::OrdMask).
#[derive(Debug, Error)]
pub enum Error {
    /// [`.key_points()`](crate::OrdMask::key_points) for [`OrdMask`](crate::OrdMask) must be non-decreasing,
    /// but value at this index is less than the preceding value.
    #[error(
        "key points for OrdMask must be non-decreasing, \
         but value at index {0} is less than the preceding value"
    )]
    FallingAt(usize),
}
