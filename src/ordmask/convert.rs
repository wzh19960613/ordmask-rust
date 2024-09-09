use super::OrdMask;

pub fn get_first_falling_index<T: Ord>(vec: &Vec<T>) -> usize {
    for i in 1..vec.len() {
        if vec[i] < vec[i - 1] {
            return i;
        }
    }
    0
}

pub struct Error {
    falling_pos: usize,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Can't convert Vec<T> to OrdMask<T> because it should be non-decreasing.\n\
            The value at index {} is less than the value at index {}.",
            self.falling_pos,
            self.falling_pos - 1,
        )
    }
}

impl<T: Ord + Clone> TryFrom<Vec<T>> for OrdMask<T> {
    type Error = Error;

    /// Convert a `Vec<T>` to an simplified `OrdMask<T>`.
    ///
    /// # Errors
    ///
    /// It will return an error if `vec` is not non-decreasing.
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        match get_first_falling_index(&vec) {
            0 => {
                let mut result = Self(vec);
                result.simplify();
                Ok(result)
            }
            n => Err(Error { falling_pos: n }),
        }
    }
}

impl<T: Ord + Clone> From<OrdMask<T>> for Vec<T> {
    fn from(mask: OrdMask<T>) -> Self {
        mask.0
    }
}

impl<T: Ord + Clone> OrdMask<T> {
    /// Create an `OrdMask` from a `Vec<T>`.
    ///
    /// # Panics
    ///
    /// It will panic if `vec` is not non-decreasing.
    /// Or you can use `try_from` to handle the error.
    pub fn from(vec: Vec<T>) -> Self {
        Self::try_from(vec).unwrap()
    }

    /// Create an `OrdMask` from a `Vec<T>`.
    ///
    /// # Safety
    ///
    /// The `vec` must be non-decreasing, otherwise the behavior is undefined.
    pub unsafe fn with_unchecked(vec: Vec<T>) -> Self {
        Self(vec)
    }
}
