// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

pub trait Primes {
    type I: Iterator<Item = Self>;
    type LI: Iterator<Item = Self>;

    fn primes_less_than(n: &Self) -> Self::LI;

    fn primes_less_than_or_equal_to(n: &Self) -> Self::LI;

    fn primes() -> Self::I;
}

pub trait IsPrime {
    fn is_prime(&self) -> bool;
}
