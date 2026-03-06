use crate::{OrdMask, WithMin};

macro_rules! impl_op {
    ($op: ident
        $(#[$meta:meta])*
        fn $fn_name:ident($self:ident, $rhs:ident) $body: block
    ) => {
        impl_op! { $op for OrdMask<T> { $( #[$meta])* fn $fn_name($self, $rhs) $body } }
        impl_op! { $op for &OrdMask<T> {
            $(#[$meta])*
            #[allow(clippy::useless_asref)]
            fn $fn_name($self, $rhs) $body
        } }
    };

    ($op: ident for $lt:ty {
        $(#[$meta:meta])*
        fn $fn_name:ident($self:ident, $rhs:ident) $body: block
    }) => {
        impl<T, R> std::ops::$op<R> for $lt
        where
            R: AsRef<OrdMask<T>>,
            T: Ord + Clone + WithMin,
        {
            type Output = OrdMask<T>;
            $(#[$meta])*
            fn $fn_name($self, $rhs: R) -> Self::Output $body
        }
    };
}

impl_op! { BitOr
    /// Create a new [`OrdMask`] representing the union of `self` and `rhs`.
    ///
    /// Equivalent to `OrdMask::union(&[self.as_ref(), rhs.as_ref()])`.
    ///
    /// Use [`OrdMask::union`] when there are more than two masks to combine.
    fn bitor(self, rhs) { OrdMask::union(&[self.as_ref(), rhs.as_ref()]) }
}

impl_op! { BitAnd
    /// Create a new [`OrdMask`] representing the intersection of `self` and `rhs`.
    ///
    /// Equivalent to `OrdMask::intersection(&[self.as_ref(), rhs.as_ref()])`.
    ///
    /// Use [`OrdMask::intersection`] when there are more than two masks to combine.
    fn bitand(self, rhs) { OrdMask::intersection(&[self.as_ref(), rhs.as_ref()]) }
}

impl_op! { BitXor
    /// Create a new [`OrdMask`] representing the symmetric difference of `self` and `rhs`.
    ///
    /// Equivalent to `self.symmetric_difference(rhs.as_ref())`.
    ///
    /// See [`OrdMask::symmetric_difference`].
    fn bitxor(self, rhs) { self.symmetric_difference(rhs.as_ref()) }
}

impl_op! { Sub
    /// Create a new [`OrdMask`] representing `self` minus `rhs`.
    ///
    /// Equivalent to `self.minus(&[rhs.as_ref()])`
    ///
    /// Use [`OrdMask::minus`] when there are more than one mask to remove.
    fn sub(self, rhs) { self.minus(&[rhs.as_ref()]) }
}

impl<T: Ord + Clone + WithMin> std::ops::Not for OrdMask<T> {
    type Output = OrdMask<T>;

    /// Consume `self` and return its complement.
    ///
    /// Equivalent to `self.to_complement()`. See [`OrdMask::to_complement`].
    fn not(self) -> Self::Output {
        self.to_complement()
    }
}

impl<T: Ord + Clone + WithMin> std::ops::Not for &OrdMask<T> {
    type Output = OrdMask<T>;

    /// Create a new [`OrdMask`] representing the complement of `self`.
    ///
    /// Equivalent to `self.complement()`. See [`OrdMask::complement`].
    fn not(self) -> Self::Output {
        self.complement()
    }
}
