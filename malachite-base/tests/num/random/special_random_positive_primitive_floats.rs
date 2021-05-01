use itertools::Itertools;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::random::special_random_positive_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::CheckedToF64;
use std::panic::catch_unwind;

fn special_random_positive_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
    mean_exponent_numerator: u64,
    mean_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
) {
    let xs = special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        mean_exponent_numerator,
        mean_exponent_denominator,
        mean_precision_numerator,
        mean_precision_denominator,
        mean_special_p_numerator,
        mean_special_p_denominator,
    );
    let actual_values = xs.clone().take(50).map(NiceFloat).collect_vec();
    let actual_common_values = common_values_map(1000000, 20, xs.clone().map(NiceFloat));
    let actual_median = median(xs.map(NiceFloat).take(1000000));
    let (lo, hi) = expected_median;
    assert_eq!(
        (
            actual_values,
            actual_common_values.as_slice(),
            actual_median,
        ),
        (
            expected_values.iter().cloned().map(NiceFloat).collect_vec(),
            expected_common_values
                .iter()
                .map(|&(x, freq)| (NiceFloat(x), freq))
                .collect_vec()
                .as_slice(),
            (NiceFloat(lo), hi.map(NiceFloat)),
        )
    );
}

#[test]
fn test_special_random_positive_primitive_floats() {
    // f32, mean abs of exponent = 1/64, mean precision = 65/64, mean special P = 1/4
    let values = &[
        f32::POSITIVE_INFINITY,
        1.0,
        1.0,
        f32::POSITIVE_INFINITY,
        1.0,
        1.0,
        f32::POSITIVE_INFINITY,
        1.0,
        f32::POSITIVE_INFINITY,
        f32::POSITIVE_INFINITY,
        1.0,
        f32::POSITIVE_INFINITY,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        0.5,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        f32::POSITIVE_INFINITY,
        1.0,
        f32::POSITIVE_INFINITY,
        f32::POSITIVE_INFINITY,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        f32::POSITIVE_INFINITY,
        1.0,
        1.0,
        0.5,
        1.0,
        f32::POSITIVE_INFINITY,
        f32::POSITIVE_INFINITY,
        f32::POSITIVE_INFINITY,
        1.0,
    ];
    let common_values = &[
        (1.0, 716179),
        (f32::POSITIVE_INFINITY, 250209),
        (1.5, 10958),
        (2.0, 10931),
        (0.5, 10886),
        (4.0, 169),
        (0.25, 168),
        (3.0, 166),
        (0.75, 159),
        (1.75, 79),
        (1.25, 72),
        (0.125, 4),
        (2.5, 3),
        (3.5, 3),
        (6.0, 3),
        (8.0, 3),
        (0.375, 3),
        (1.375, 2),
        (7.0, 1),
        (0.625, 1),
    ];
    let sample_median = (1.0, None);
    special_random_positive_primitive_floats_helper::<f32>(
        1,
        64,
        65,
        64,
        1,
        4,
        values,
        common_values,
        sample_median,
    );

    // f32, mean abs of exponent = 1, mean precision = 2, mean special P = 1/10
    let values = &[
        1.25,
        1.125,
        3.5,
        f32::POSITIVE_INFINITY,
        1.0,
        1.0,
        2.0,
        2.5,
        1.5,
        2.0,
        1.0,
        3.0,
        2.0,
        f32::POSITIVE_INFINITY,
        5.109375,
        1.25,
        1.5,
        f32::POSITIVE_INFINITY,
        3.5,
        0.1875,
        1.0,
        0.25,
        1.5,
        6.0,
        6.0,
        6.0,
        0.5,
        0.125,
        1.0,
        f32::POSITIVE_INFINITY,
        0.1875,
        4.0,
        f32::POSITIVE_INFINITY,
        0.625,
        5.0,
        7.25,
        3.5,
        0.125,
        2.0,
        0.5,
        0.875,
        7.0,
        16.0,
        16.0,
        3.0,
        1.0,
        f32::POSITIVE_INFINITY,
        1.0,
        8.0,
        4.0,
    ];
    let common_values = &[
        (1.0, 149686),
        (f32::POSITIVE_INFINITY, 100224),
        (0.5, 75093),
        (1.5, 75069),
        (2.0, 74980),
        (3.0, 37539),
        (0.75, 37494),
        (4.0, 37403),
        (0.25, 37389),
        (6.0, 18880),
        (1.75, 18825),
        (1.25, 18791),
        (0.375, 18746),
        (0.125, 18725),
        (8.0, 18641),
        (16.0, 9453),
        (2.5, 9437),
        (0.625, 9374),
        (12.0, 9347),
        (0.0625, 9338),
    ];
    let sample_median = (1.5, None);
    special_random_positive_primitive_floats_helper::<f32>(
        1,
        1,
        2,
        1,
        1,
        10,
        values,
        common_values,
        sample_median,
    );

    // f32, mean abs of exponent = 10, mean precision = 10, mean special P = 1/100
    let values = &[
        0.5688782,
        0.00000166893,
        0.015167236,
        0.75,
        92864.0,
        0.012998581,
        0.02949524,
        3.28125,
        7.5703125,
        0.00005722046,
        0.33540344,
        18432.0,
        61440.0,
        1.4375,
        50.0,
        0.4140625,
        0.001739502,
        1388.125,
        0.15625,
        0.001373291,
        2.7008355e-8,
        0.03930664,
        7995392.0,
        0.23046875,
        203.5,
        7.0,
        1048576.0,
        1.1920929e-7,
        0.4765625,
        0.0065612793,
        46137344.0,
        4096.0,
        18.8125,
        6.0,
        6.5664062,
        2.1781006,
        0.0000038146973,
        0.022613525,
        0.000061035156,
        0.0000440937,
        0.00015640259,
        33.0,
        0.0043945312,
        0.013641357,
        30.759766,
        26.0,
        0.19921875,
        230.125,
        0.078125,
        4352.0,
    ];
    let common_values = &[
        (f32::POSITIVE_INFINITY, 9989),
        (1.0, 5108),
        (0.5, 4697),
        (2.0, 4617),
        (1.5, 4581),
        (3.0, 4216),
        (0.25, 4158),
        (4.0, 4153),
        (0.75, 4126),
        (0.125, 3954),
        (0.375, 3890),
        (6.0, 3793),
        (8.0, 3783),
        (0.0625, 3492),
        (0.1875, 3479),
        (16.0, 3449),
        (12.0, 3443),
        (32.0, 3229),
        (0.09375, 3146),
        (0.03125, 3117),
    ];
    let sample_median = (1.5, None);
    special_random_positive_primitive_floats_helper::<f32>(
        10,
        1,
        10,
        1,
        1,
        100,
        values,
        common_values,
        sample_median,
    );

    // f64, mean abs of exponent = 1/64, mean precision = 65/64, mean special P = 1/4
    let values = &[
        f64::POSITIVE_INFINITY,
        1.0,
        1.0,
        f64::POSITIVE_INFINITY,
        1.0,
        1.0,
        f64::POSITIVE_INFINITY,
        1.0,
        f64::POSITIVE_INFINITY,
        f64::POSITIVE_INFINITY,
        1.0,
        f64::POSITIVE_INFINITY,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        0.5,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        f64::POSITIVE_INFINITY,
        1.0,
        f64::POSITIVE_INFINITY,
        f64::POSITIVE_INFINITY,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        f64::POSITIVE_INFINITY,
        1.0,
        1.0,
        0.5,
        1.0,
        f64::POSITIVE_INFINITY,
        f64::POSITIVE_INFINITY,
        f64::POSITIVE_INFINITY,
        1.0,
    ];
    let common_values = &[
        (1.0, 716179),
        (f64::POSITIVE_INFINITY, 250209),
        (1.5, 10958),
        (2.0, 10931),
        (0.5, 10886),
        (4.0, 169),
        (0.25, 168),
        (3.0, 166),
        (0.75, 159),
        (1.75, 79),
        (1.25, 72),
        (0.125, 4),
        (2.5, 3),
        (3.5, 3),
        (6.0, 3),
        (8.0, 3),
        (0.375, 3),
        (1.375, 2),
        (7.0, 1),
        (0.625, 1),
    ];
    let sample_median = (1.0, None);
    special_random_positive_primitive_floats_helper::<f64>(
        1,
        64,
        65,
        64,
        1,
        4,
        values,
        common_values,
        sample_median,
    );

    // f64, mean abs of exponent = 1, mean precision = 2, mean special P = 1/10
    let values = &[
        1.25,
        1.125,
        3.5,
        f64::POSITIVE_INFINITY,
        1.0,
        1.0,
        2.0,
        2.5,
        1.5,
        2.0,
        1.0,
        3.0,
        2.0,
        f64::POSITIVE_INFINITY,
        5.109375,
        1.25,
        1.5,
        f64::POSITIVE_INFINITY,
        3.5,
        0.1875,
        1.0,
        0.25,
        1.5,
        6.0,
        6.0,
        6.0,
        0.5,
        0.125,
        1.0,
        f64::POSITIVE_INFINITY,
        0.1875,
        4.0,
        f64::POSITIVE_INFINITY,
        0.625,
        5.0,
        7.25,
        3.5,
        0.125,
        2.0,
        0.5,
        0.875,
        7.0,
        16.0,
        16.0,
        3.0,
        1.0,
        f64::POSITIVE_INFINITY,
        1.0,
        8.0,
        4.0,
    ];
    let common_values = &[
        (1.0, 149686),
        (f64::POSITIVE_INFINITY, 100224),
        (0.5, 75093),
        (1.5, 75069),
        (2.0, 74979),
        (3.0, 37539),
        (0.75, 37494),
        (4.0, 37403),
        (0.25, 37389),
        (6.0, 18880),
        (1.75, 18871),
        (0.375, 18746),
        (1.25, 18745),
        (0.125, 18725),
        (8.0, 18641),
        (16.0, 9453),
        (2.5, 9434),
        (0.875, 9360),
        (12.0, 9347),
        (0.0625, 9338),
    ];
    let sample_median = (1.5, None);
    special_random_positive_primitive_floats_helper::<f64>(
        1,
        1,
        2,
        1,
        1,
        10,
        values,
        common_values,
        sample_median,
    );

    // f64, mean abs of exponent = 10, mean precision = 10, mean special P = 1/100
    let values = &[
        0.568878173828125,
        1.6689300537109375e-6,
        0.015167236328125,
        0.75,
        92864.0,
        0.012998580932617188,
        0.02291788616003032,
        2.762363215908408,
        6.1796875,
        0.000041961669921875,
        0.4728546142578125,
        22528.0,
        61440.0,
        1.4375,
        38.0,
        0.4296875,
        0.0013771892390650464,
        1069.625,
        0.21875,
        0.001739501953125,
        1.7695128917694092e-8,
        0.055419921875,
        5111808.0,
        0.23828125,
        136.5,
        5.0,
        1048576.0,
        1.1920928955078125e-7,
        0.3828125,
        0.006805419921875,
        54525952.0,
        4096.0,
        22.0625,
        6.0,
        6.43359375,
        2.0997314453125,
        3.814697265625e-6,
        0.029205322265625,
        0.00006103515625,
        0.00003288569860160351,
        0.000209808349609375,
        37.0,
        0.00537109375,
        0.008087158203125,
        21.623046875,
        26.0,
        0.17578125,
        242.625,
        0.078125,
        5376.0,
    ];
    let common_values = &[
        (f64::POSITIVE_INFINITY, 9989),
        (1.0, 4772),
        (0.5, 4327),
        (1.5, 4275),
        (2.0, 4261),
        (3.0, 3913),
        (4.0, 3816),
        (0.25, 3816),
        (0.75, 3780),
        (0.125, 3640),
        (0.375, 3629),
        (8.0, 3537),
        (6.0, 3493),
        (0.0625, 3253),
        (12.0, 3209),
        (16.0, 3195),
        (0.1875, 3164),
        (32.0, 2958),
        (0.03125, 2899),
        (0.09375, 2874),
    ];
    let sample_median = (1.5088882446289062, Some(1.50901460647583));
    special_random_positive_primitive_floats_helper::<f64>(
        10,
        1,
        10,
        1,
        1,
        100,
        values,
        common_values,
        sample_median,
    );
}

fn special_random_positive_primitive_floats_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        0,
        1,
        10,
        1,
        1,
        10
    ));
    assert_panic!(special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        1,
        0,
        10,
        1,
        1,
        10
    ));
    assert_panic!(special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        1,
        1,
        10
    ));
    assert_panic!(special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        0,
        1,
        10
    ));
    assert_panic!(special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        10,
        1,
        1,
        0
    ));
    assert_panic!(special_random_positive_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        10,
        1,
        2,
        1
    ));
}

#[test]
fn special_random_positive_primitive_floats_fail() {
    apply_fn_to_primitive_floats!(special_random_positive_primitive_floats_fail_helper);
}
