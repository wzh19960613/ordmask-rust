use super::OrdMask;

pub fn get_first_falling_index<T: PartialOrd>(vec: &Vec<T>) -> usize {
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

impl<T: Ord + Clone> From<OrdMask<T>> for Vec<T> {
    fn from(mask: OrdMask<T>) -> Self {
        mask.key_points
    }
}

impl<T: Ord + Clone> TryFrom<Vec<T>> for OrdMask<T> {
    type Error = Error;

    fn try_from(key_points: Vec<T>) -> Result<Self, Self::Error> {
        Self::try_new(key_points, false)
    }
}

impl<T: Ord + Clone> OrdMask<T> {
    fn try_new(key_points: Vec<T>, reversed: bool) -> Result<Self, Error> {
        match get_first_falling_index(&key_points) {
            0 => {
                let mut result = Self {
                    key_points,
                    reversed,
                };
                result.simplify();
                Ok(result)
            }
            n => Err(Error { falling_pos: n }),
        }
    }

    /// Create an `OrdMask` from a `Vec<T>`.
    ///
    /// # Panics
    ///
    /// It will panic if `vec` is not non-decreasing.
    /// Or you can use `try_from` to handle the error.
    pub fn from(key_points: Vec<T>) -> Self {
        Self::try_new(key_points, false).unwrap()
    }

    /// Create an `OrdMask` from a `Vec<T>` that is the complement of the original mask.
    ///
    /// # Panics
    ///
    /// It will panic if `vec` is not non-decreasing.
    /// Or you can use `try_from` to handle the error.
    pub fn from_complement(key_points: Vec<T>) -> Self {
        Self::try_new(key_points, true).unwrap()
    }

    /// Create an `OrdMask` from a `Vec<T>`.
    ///
    /// # Safety
    ///
    /// The `vec` must be non-decreasing, otherwise the behavior is undefined.
    pub unsafe fn with_unchecked(key_points: Vec<T>, reversed: bool) -> Self {
        Self {
            key_points,
            reversed,
        }
    }
}
