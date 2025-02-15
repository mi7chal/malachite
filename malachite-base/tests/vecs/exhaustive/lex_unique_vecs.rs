// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::test_util::vecs::exhaustive::{
    exhaustive_vecs_helper_helper, exhaustive_vecs_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::lex_unique_vecs;
use std::fmt::Debug;

fn lex_unique_vecs_helper<I: Clone + Iterator>(xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(lex_unique_vecs(xs), out);
}

fn lex_unique_vecs_small_helper<I: Clone + Iterator>(xs: I, out_len: usize, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(lex_unique_vecs(xs), out_len, out);
}

#[test]
fn test_lex_unique_vecs() {
    lex_unique_vecs_small_helper(nevers(), 1, &[&[]]);
    lex_unique_vecs_small_helper(exhaustive_units(), 2, &[&[], &[()]]);
    lex_unique_vecs_small_helper(
        exhaustive_bools(),
        5,
        &[&[], &[false], &[false, true], &[true], &[true, false]],
    );
    lex_unique_vecs_small_helper(
        1..=6,
        1957,
        &[
            &[],
            &[1],
            &[1, 2],
            &[1, 2, 3],
            &[1, 2, 3, 4],
            &[1, 2, 3, 4, 5],
            &[1, 2, 3, 4, 5, 6],
            &[1, 2, 3, 4, 6],
            &[1, 2, 3, 4, 6, 5],
            &[1, 2, 3, 5],
            &[1, 2, 3, 5, 4],
            &[1, 2, 3, 5, 4, 6],
            &[1, 2, 3, 5, 6],
            &[1, 2, 3, 5, 6, 4],
            &[1, 2, 3, 6],
            &[1, 2, 3, 6, 4],
            &[1, 2, 3, 6, 4, 5],
            &[1, 2, 3, 6, 5],
            &[1, 2, 3, 6, 5, 4],
            &[1, 2, 4],
        ],
    );
    lex_unique_vecs_small_helper(
        'a'..='c',
        16,
        &[
            &[],
            &['a'],
            &['a', 'b'],
            &['a', 'b', 'c'],
            &['a', 'c'],
            &['a', 'c', 'b'],
            &['b'],
            &['b', 'a'],
            &['b', 'a', 'c'],
            &['b', 'c'],
            &['b', 'c', 'a'],
            &['c'],
            &['c', 'a'],
            &['c', 'a', 'b'],
            &['c', 'b'],
            &['c', 'b', 'a'],
        ],
    );
    lex_unique_vecs_helper(
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['a', 'b'],
            &['a', 'b', 'c'],
            &['a', 'b', 'c', 'd'],
            &['a', 'b', 'c', 'd', 'e'],
            &['a', 'b', 'c', 'd', 'e', 'f'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'],
            &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q'],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r',
            ],
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's',
            ],
        ],
    );
}
