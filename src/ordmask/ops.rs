use super::OrdMask;

macro_rules! impl_bitor {
    ($lt:ty, $rt:ty) => {
        impl<T: Ord + Clone> std::ops::BitOr<$rt> for $lt {
            type Output = OrdMask<T>;

            /// Create a new OrdMask representing the union of the `self` and `rhs`.
            ///
            /// Values included in the union must be included in
            /// at least one of the `self` or `rhs`.
            fn bitor(self, rhs: $rt) -> Self::Output {
                OrdMask::union(&[&self, &rhs])
            }
        }
    };
}

impl_bitor!(OrdMask<T>, OrdMask<T>);
impl_bitor!(OrdMask<T>, &OrdMask<T>);
impl_bitor!(&OrdMask<T>, OrdMask<T>);
impl_bitor!(&OrdMask<T>, &OrdMask<T>);

macro_rules! impl_bitand {
    ($lt:ty, $rt:ty) => {
        impl<T: Ord + Clone> std::ops::BitAnd<$rt> for $lt {
            type Output = OrdMask<T>;

            /// Create a new OrdMask representing the intersection of the `self` and `rhs`.
            ///
            /// Values included in the intersection must be included in all of the `self` and `rhs`.
            fn bitand(self, rhs: $rt) -> Self::Output {
                OrdMask::intersection(&[&self, &rhs])
            }
        }
    };
}

impl_bitand!(OrdMask<T>, OrdMask<T>);
impl_bitand!(OrdMask<T>, &OrdMask<T>);
impl_bitand!(&OrdMask<T>, OrdMask<T>);
impl_bitand!(&OrdMask<T>, &OrdMask<T>);

macro_rules! impl_bitxor {
    ($lt:ty, $rt:ty) => {
        impl<T: Ord + Clone> std::ops::BitXor<$rt> for $lt {
            type Output = OrdMask<T>;

            /// Create a new OrdMask representing the symmetric difference of the `self` and `rhs`.
            ///
            /// Values included in the symmetric difference must be included in
            /// one of the `self` or `rhs`, but not both.
            fn bitxor(self, rhs: $rt) -> Self::Output {
                self.symmetric_difference(&rhs)
            }
        }
    };
}

impl_bitxor!(OrdMask<T>, OrdMask<T>);
impl_bitxor!(OrdMask<T>, &OrdMask<T>);
impl_bitxor!(&OrdMask<T>, OrdMask<T>);
impl_bitxor!(&OrdMask<T>, &OrdMask<T>);

macro_rules! impl_sub {
    ($lt:ty, $rt:ty) => {
        impl<T: Ord + Clone> std::ops::Sub<$rt> for $lt {
            type Output = OrdMask<T>;

            /// Create a new OrdMask representing the difference of the `self` and `rhs`.
            ///
            /// Values included in the difference must be included in `self` and excluded in `rhs`.
            fn sub(self, rhs: $rt) -> Self::Output {
                self.minus(&[&rhs])
            }
        }
    };
}

impl_sub!(OrdMask<T>, OrdMask<T>);
impl_sub!(OrdMask<T>, &OrdMask<T>);
impl_sub!(&OrdMask<T>, OrdMask<T>);
impl_sub!(&OrdMask<T>, &OrdMask<T>);

impl<T: Ord + Clone> std::ops::Not for OrdMask<T> {
    type Output = OrdMask<T>;

    /// Create a new OrdMask representing the complement of the `self`.
    ///
    /// Values included in the complement must be excluded in `self`.
    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl<T: Ord + Clone> std::ops::Not for &OrdMask<T> {
    type Output = OrdMask<T>;

    /// Create a new OrdMask representing the complement of the `self`.
    ///
    /// Values included in the complement must be excluded in `self`.
    fn not(self) -> Self::Output {
        self.new_complement()
    }
}
