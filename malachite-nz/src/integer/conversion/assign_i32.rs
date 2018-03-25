use integer::Integer;
use malachite_base::num::{Assign, UnsignedAbs};

/// Assigns an `i32` to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Assign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(456);
///     x.assign(-123);
///     assert_eq!(x.to_string(), "-123");
/// }
/// ```
impl Assign<i32> for Integer {
    fn assign(&mut self, other: i32) {
        self.sign = other >= 0;
        self.abs.assign(other.unsigned_abs());
    }
}
