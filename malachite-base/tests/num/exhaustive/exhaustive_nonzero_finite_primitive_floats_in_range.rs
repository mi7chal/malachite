// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::exhaustive::exhaustive_nonzero_finite_primitive_floats_in_range;
use malachite_base::test_util::num::exhaustive::exhaustive_primitive_floats_helper_helper;
use std::panic::catch_unwind;

fn exhaustive_nonzero_finite_primitive_floats_in_range_helper<T: PrimitiveFloat>(
    a: T,
    b: T,
    out: &[T],
) {
    exhaustive_primitive_floats_helper_helper(
        exhaustive_nonzero_finite_primitive_floats_in_range::<T>(a, b),
        out,
    );
}

#[test]
fn test_exhaustive_nonzero_finite_primitive_floats_in_range() {
    exhaustive_nonzero_finite_primitive_floats_in_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        &[
            3.0, 2.75, 2.875, 3.125, 2.8125, 2.9375, 3.0625, 2.71875, 2.78125, 2.84375, 2.90625,
            2.96875, 3.03125, 3.09375, 2.734375, 2.765625, 2.796875, 2.828125, 2.859375, 2.890625,
            2.921875, 2.953125, 2.984375, 3.015625, 3.046875, 3.078125, 3.109375, 3.140625,
            2.7265625, 2.7421875, 2.7578125, 2.7734375, 2.7890625, 2.8046875, 2.8203125, 2.8359375,
            2.8515625, 2.8671875, 2.8828125, 2.8984375, 2.9140625, 2.9296875, 2.9453125, 2.9609375,
            2.9765625, 2.9921875, 3.0078125, 3.0234375, 3.0390625, 3.0546875,
        ],
    );
    exhaustive_nonzero_finite_primitive_floats_in_range_helper::<f32>(5.0, 5.0, &[5.0]);
    exhaustive_nonzero_finite_primitive_floats_in_range_helper::<f32>(
        1.0,
        6.0,
        &[
            1.0, 2.0, 1.5, 4.0, 1.25, 3.0, 1.75, 1.125, 1.375, 2.5, 1.625, 6.0, 1.875, 3.5, 1.0625,
            2.25, 1.1875, 2.75, 1.3125, 5.0, 1.4375, 3.25, 1.5625, 1.6875, 1.8125, 3.75, 1.9375,
            4.5, 1.03125, 2.125, 1.09375, 5.5, 1.15625, 2.375, 1.21875, 4.25, 1.28125, 2.625,
            1.34375, 1.40625, 1.46875, 2.875, 1.53125, 4.75, 1.59375, 3.125, 1.65625, 3.375,
            1.71875, 3.625,
        ],
    );
    exhaustive_nonzero_finite_primitive_floats_in_range_helper::<f32>(
        -6.0,
        -1.0,
        &[
            -1.0, -2.0, -1.5, -4.0, -1.25, -3.0, -1.75, -1.125, -1.375, -2.5, -1.625, -6.0, -1.875,
            -3.5, -1.0625, -2.25, -1.1875, -2.75, -1.3125, -5.0, -1.4375, -3.25, -1.5625, -1.6875,
            -1.8125, -3.75, -1.9375, -4.5, -1.03125, -2.125, -1.09375, -5.5, -1.15625, -2.375,
            -1.21875, -4.25, -1.28125, -2.625, -1.34375, -1.40625, -1.46875, -2.875, -1.53125,
            -4.75, -1.59375, -3.125, -1.65625, -3.375, -1.71875, -3.625,
        ],
    );
    exhaustive_nonzero_finite_primitive_floats_in_range_helper::<f32>(
        -6.0,
        6.0,
        &[
            1.0, -1.0, 2.0, -2.0, 1.5, -1.5, 0.5, -0.5, 1.25, -1.25, 3.0, -3.0, 1.75, -1.75, 4.0,
            -4.0, 1.125, -1.125, 2.5, -2.5, 1.375, -1.375, 0.75, -0.75, 1.625, -1.625, 3.5, -3.5,
            1.875, -1.875, 0.25, -0.25, 1.0625, -1.0625, 2.25, -2.25, 1.1875, -1.1875, 0.625,
            -0.625, 1.3125, -1.3125, 2.75, -2.75, 1.4375, -1.4375, 6.0, -6.0, 1.5625, -1.5625,
        ],
    );
    exhaustive_nonzero_finite_primitive_floats_in_range_helper::<f32>(
        -0.1,
        10.0,
        &[
            1.0,
            -0.0625,
            2.0,
            -0.03125,
            1.5,
            -0.09375,
            0.5,
            -0.015625,
            1.25,
            -0.078125,
            3.0,
            -0.046875,
            1.75,
            -0.0703125,
            4.0,
            -0.0078125,
            1.125,
            -0.0859375,
            2.5,
            -0.0390625,
            1.375,
            -0.06640625,
            0.75,
            -0.0234375,
            1.625,
            -0.07421875,
            3.5,
            -0.0546875,
            1.875,
            -0.08203125,
            0.25,
            -0.00390625,
            1.0625,
            -0.08984375,
            2.25,
            -0.03515625,
            1.1875,
            -0.09765625,
            0.625,
            -0.01953125,
            1.3125,
            -0.064453125,
            2.75,
            -0.04296875,
            1.4375,
            -0.068359375,
            6.0,
            -0.01171875,
            1.5625,
            -0.072265625,
        ],
    );

    exhaustive_nonzero_finite_primitive_floats_in_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        &[
            3.0, 2.75, 2.875, 3.125, 2.8125, 2.9375, 3.0625, 2.71875, 2.78125, 2.84375, 2.90625,
            2.96875, 3.03125, 3.09375, 2.734375, 2.765625, 2.796875, 2.828125, 2.859375, 2.890625,
            2.921875, 2.953125, 2.984375, 3.015625, 3.046875, 3.078125, 3.109375, 3.140625,
            2.7265625, 2.7421875, 2.7578125, 2.7734375, 2.7890625, 2.8046875, 2.8203125, 2.8359375,
            2.8515625, 2.8671875, 2.8828125, 2.8984375, 2.9140625, 2.9296875, 2.9453125, 2.9609375,
            2.9765625, 2.9921875, 3.0078125, 3.0234375, 3.0390625, 3.0546875,
        ],
    );
    exhaustive_nonzero_finite_primitive_floats_in_range_helper::<f64>(5.0, 5.0, &[5.0]);
    exhaustive_nonzero_finite_primitive_floats_in_range_helper::<f64>(
        1.0,
        6.0,
        &[
            1.0, 2.0, 1.5, 4.0, 1.25, 3.0, 1.75, 1.125, 1.375, 2.5, 1.625, 6.0, 1.875, 3.5, 1.0625,
            2.25, 1.1875, 2.75, 1.3125, 5.0, 1.4375, 3.25, 1.5625, 1.6875, 1.8125, 3.75, 1.9375,
            4.5, 1.03125, 2.125, 1.09375, 5.5, 1.15625, 2.375, 1.21875, 4.25, 1.28125, 2.625,
            1.34375, 1.40625, 1.46875, 2.875, 1.53125, 4.75, 1.59375, 3.125, 1.65625, 3.375,
            1.71875, 3.625,
        ],
    );
    exhaustive_nonzero_finite_primitive_floats_in_range_helper::<f64>(
        -6.0,
        -1.0,
        &[
            -1.0, -2.0, -1.5, -4.0, -1.25, -3.0, -1.75, -1.125, -1.375, -2.5, -1.625, -6.0, -1.875,
            -3.5, -1.0625, -2.25, -1.1875, -2.75, -1.3125, -5.0, -1.4375, -3.25, -1.5625, -1.6875,
            -1.8125, -3.75, -1.9375, -4.5, -1.03125, -2.125, -1.09375, -5.5, -1.15625, -2.375,
            -1.21875, -4.25, -1.28125, -2.625, -1.34375, -1.40625, -1.46875, -2.875, -1.53125,
            -4.75, -1.59375, -3.125, -1.65625, -3.375, -1.71875, -3.625,
        ],
    );
    exhaustive_nonzero_finite_primitive_floats_in_range_helper::<f64>(
        -6.0,
        6.0,
        &[
            1.0, -1.0, 2.0, -2.0, 1.5, -1.5, 0.5, -0.5, 1.25, -1.25, 3.0, -3.0, 1.75, -1.75, 4.0,
            -4.0, 1.125, -1.125, 2.5, -2.5, 1.375, -1.375, 0.75, -0.75, 1.625, -1.625, 3.5, -3.5,
            1.875, -1.875, 0.25, -0.25, 1.0625, -1.0625, 2.25, -2.25, 1.1875, -1.1875, 0.625,
            -0.625, 1.3125, -1.3125, 2.75, -2.75, 1.4375, -1.4375, 6.0, -6.0, 1.5625, -1.5625,
        ],
    );
    exhaustive_nonzero_finite_primitive_floats_in_range_helper::<f64>(
        -0.1,
        10.0,
        &[
            1.0,
            -0.0625,
            2.0,
            -0.03125,
            1.5,
            -0.09375,
            0.5,
            -0.015625,
            1.25,
            -0.078125,
            3.0,
            -0.046875,
            1.75,
            -0.0703125,
            4.0,
            -0.0078125,
            1.125,
            -0.0859375,
            2.5,
            -0.0390625,
            1.375,
            -0.06640625,
            0.75,
            -0.0234375,
            1.625,
            -0.07421875,
            3.5,
            -0.0546875,
            1.875,
            -0.08203125,
            0.25,
            -0.00390625,
            1.0625,
            -0.08984375,
            2.25,
            -0.03515625,
            1.1875,
            -0.09765625,
            0.625,
            -0.01953125,
            1.3125,
            -0.064453125,
            2.75,
            -0.04296875,
            1.4375,
            -0.068359375,
            6.0,
            -0.01171875,
            1.5625,
            -0.072265625,
        ],
    );
}

fn exhaustive_nonzero_finite_primitive_floats_in_range_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(exhaustive_nonzero_finite_primitive_floats_in_range::<T>(
        T::from(1.2),
        T::from(1.1),
    ));
    assert_panic!(exhaustive_nonzero_finite_primitive_floats_in_range::<T>(
        T::ONE,
        T::INFINITY,
    ));
    assert_panic!(exhaustive_nonzero_finite_primitive_floats_in_range::<T>(
        T::ONE,
        T::NAN,
    ));
}

#[test]
fn exhaustive_nonzero_finite_primitive_floats_in_range_fail() {
    apply_fn_to_primitive_floats!(exhaustive_nonzero_finite_primitive_floats_in_range_fail_helper);
}
