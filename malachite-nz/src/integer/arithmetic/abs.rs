use integer::Integer;
use malachite_base::num::{Abs, AbsAssign, UnsignedAbs};
use natural::Natural;

impl Abs for Integer {
    type Output = Integer;

    /// Finds the absolute value of an `Integer`, taking the `Integer` by value.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{Abs, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.abs().to_string(), "0");
    ///     assert_eq!(Integer::from(123).abs().to_string(), "123");
    ///     assert_eq!(Integer::from(-123).abs().to_string(), "123");
    /// }
    /// ```
    fn abs(mut self) -> Integer {
        self.sign = true;
        self
    }
}

impl<'a> Abs for &'a Integer {
    type Output = Integer;

    /// Finds the absolute value of an `Integer`, taking the `Integer` by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{Abs, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((&Integer::ZERO).abs().to_string(), "0");
    ///     assert_eq!((&Integer::from(123)).abs().to_string(), "123");
    ///     assert_eq!((&Integer::from(-123)).abs().to_string(), "123");
    /// }
    /// ```
    fn abs(self) -> Integer {
        Integer {
            sign: true,
            abs: self.abs.clone(),
        }
    }
}

impl UnsignedAbs for Integer {
    type Output = Natural;

    /// Finds the absolute value of an `Integer`, taking the `Integer` by value and converting the
    /// result to a `Natural`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{UnsignedAbs, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.unsigned_abs().to_string(), "0");
    ///     assert_eq!(Integer::from(123).unsigned_abs().to_string(), "123");
    ///     assert_eq!(Integer::from(-123).unsigned_abs().to_string(), "123");
    /// }
    /// ```
    fn unsigned_abs(self) -> Natural {
        self.abs
    }
}

impl<'a> UnsignedAbs for &'a Integer {
    type Output = Natural;

    /// Finds the absolute value of an `Integer`, taking the `Integer` by reference and converting
    /// the result to a `Natural`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{UnsignedAbs, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!((&Integer::ZERO).unsigned_abs().to_string(), "0");
    ///     assert_eq!((&Integer::from(123)).unsigned_abs().to_string(), "123");
    ///     assert_eq!((&Integer::from(-123)).unsigned_abs().to_string(), "123");
    /// }
    /// ```
    fn unsigned_abs(self) -> Natural {
        self.abs.clone()
    }
}

/// Replaces an `Integer` with its absolute value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{AbsAssign, Zero};
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x.abs_assign();
///     assert_eq!(x.to_string(), "0");
///
///     let mut x = Integer::from(123);
///     x.abs_assign();
///     assert_eq!(x.to_string(), "123");
///
///     let mut x = Integer::from(-123);
///     x.abs_assign();
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl AbsAssign for Integer {
    fn abs_assign(&mut self) {
        self.sign = true;
    }
}
