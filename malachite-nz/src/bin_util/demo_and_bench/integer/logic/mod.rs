// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    and::register(runner);
    assign_bit::register(runner);
    assign_bits::register(runner);
    bits::register(runner);
    checked_count_ones::register(runner);
    checked_count_zeros::register(runner);
    checked_hamming_distance::register(runner);
    clear_bit::register(runner);
    flip_bit::register(runner);
    from_bits::register(runner);
    get_bit::register(runner);
    get_bits::register(runner);
    index_of_next_false_bit::register(runner);
    index_of_next_true_bit::register(runner);
    low_mask::register(runner);
    not::register(runner);
    or::register(runner);
    set_bit::register(runner);
    significant_bits::register(runner);
    to_bits::register(runner);
    trailing_zeros::register(runner);
    xor::register(runner);
}

mod and;
mod assign_bit;
mod assign_bits;
mod bits;
mod checked_count_ones;
mod checked_count_zeros;
mod checked_hamming_distance;
mod clear_bit;
mod flip_bit;
mod from_bits;
mod get_bit;
mod get_bits;
mod index_of_next_false_bit;
mod index_of_next_true_bit;
mod low_mask;
mod not;
mod or;
mod set_bit;
mod significant_bits;
mod to_bits;
mod trailing_zeros;
mod xor;
