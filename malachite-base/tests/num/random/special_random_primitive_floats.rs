use itertools::Itertools;
use malachite_base::num::float::nice_float::NiceFloat;
use malachite_base::num::float::PrimitiveFloat;
use malachite_base::num::random::special_random_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;
use malachite_base_test_util::stats::moments::CheckedToF64;
use std::panic::catch_unwind;

fn special_random_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
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
    let xs = special_random_primitive_floats::<T>(
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
fn test_special_random_primitive_floats() {
    // f32, mean abs of exponent = 1/64, mean precision = 65/64, mean special P = 1/4
    let values = &[
        f32::POSITIVE_INFINITY,
        1.0,
        1.0,
        f32::POSITIVE_INFINITY,
        1.0,
        -1.0,
        f32::POSITIVE_INFINITY,
        -1.0,
        f32::POSITIVE_INFINITY,
        0.0,
        -1.0,
        f32::POSITIVE_INFINITY,
        -1.0,
        1.0,
        1.0,
        -1.0,
        -1.0,
        -1.0,
        -1.0,
        -1.0,
        -1.0,
        1.0,
        -1.0,
        -1.0,
        1.0,
        0.5,
        1.0,
        -1.0,
        1.0,
        -1.0,
        1.0,
        1.0,
        0.0,
        -1.0,
        f32::NEGATIVE_INFINITY,
        0.0,
        -1.0,
        1.0,
        -1.0,
        -1.0,
        -1.0,
        -0.0,
        1.0,
        -1.0,
        -0.5,
        -1.0,
        f32::POSITIVE_INFINITY,
        f32::NEGATIVE_INFINITY,
        f32::POSITIVE_INFINITY,
        1.0,
    ];
    let common_values = &[
        (1.0, 358273),
        (-1.0, 357906),
        (f32::POSITIVE_INFINITY, 50304),
        (f32::NEGATIVE_INFINITY, 50127),
        (-0.0, 49984),
        (f32::NAN, 49926),
        (0.0, 49868),
        (2.0, 5548),
        (0.5, 5531),
        (1.5, 5489),
        (-1.5, 5469),
        (-2.0, 5383),
        (-0.5, 5355),
        (-4.0, 94),
        (-0.75, 90),
        (-0.25, 89),
        (3.0, 86),
        (-3.0, 80),
        (0.25, 79),
        (4.0, 75),
    ];
    let sample_median = (f32::NAN, None);
    special_random_primitive_floats_helper::<f32>(
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
        1.375,
        2.5,
        f32::POSITIVE_INFINITY,
        -1.0,
        -1.0,
        -2.0,
        -3.5,
        1.5,
        2.0,
        -1.0,
        -3.0,
        -2.0,
        f32::POSITIVE_INFINITY,
        -6.671875,
        -1.75,
        -1.5,
        f32::POSITIVE_INFINITY,
        3.5,
        -0.1875,
        -1.0,
        0.25,
        1.5,
        6.0,
        -6.0,
        6.0,
        -0.5,
        0.125,
        1.0,
        f32::POSITIVE_INFINITY,
        -0.1875,
        -4.0,
        0.0,
        0.875,
        -7.0,
        -6.75,
        -2.5,
        0.125,
        -2.0,
        -0.5,
        -0.875,
        5.0,
        -16.0,
        16.0,
        -3.0,
        1.0,
        f32::POSITIVE_INFINITY,
        -1.0,
        8.0,
        4.0,
    ];
    let common_values = &[
        (1.0, 74871),
        (-1.0, 74815),
        (-0.5, 37712),
        (2.0, 37667),
        (1.5, 37661),
        (-1.5, 37408),
        (0.5, 37381),
        (-2.0, 37313),
        (f32::NEGATIVE_INFINITY, 20096),
        (0.0, 20064),
        (f32::POSITIVE_INFINITY, 20042),
        (-0.0, 20022),
        (f32::NAN, 20000),
        (3.0, 18789),
        (-0.75, 18765),
        (-3.0, 18750),
        (0.75, 18729),
        (-0.25, 18729),
        (-4.0, 18710),
        (4.0, 18693),
    ];
    let sample_median = (f32::NAN, None);
    special_random_primitive_floats_helper::<f32>(
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
        0.9171448,
        0.00000166893,
        0.013153076,
        -0.75,
        -102720.0,
        -0.012674332,
        -0.022476196,
        2.65625,
        4.2109375,
        -0.000049591064,
        -0.3943634,
        -18432.0,
        -45056.0,
        -1.4375,
        -58.0,
        0.3046875,
        -0.0018005371,
        -1302.625,
        0.21875,
        0.0018005371,
        2.7008355e-8,
        -0.04321289,
        4325376.0,
        -0.23828125,
        133.5,
        5.0,
        -1048576.0,
        -1.1920929e-7,
        0.3046875,
        -0.0062561035,
        -54525950.0,
        -4096.0,
        21.9375,
        -6.0,
        -6.1132812,
        -3.1497803,
        0.0000038146973,
        -0.027618408,
        0.000061035156,
        -0.00005314802,
        0.00020980835,
        -47.0,
        0.0043945312,
        0.009185791,
        22.78711,
        18.0,
        -0.19140625,
        -232.625,
        -0.078125,
        -6912.0,
    ];
    let common_values = &[
        (1.0, 2570),
        (-1.0, 2538),
        (-2.0, 2404),
        (-1.5, 2369),
        (0.5, 2354),
        (-0.5, 2343),
        (2.0, 2213),
        (1.5, 2212),
        (3.0, 2116),
        (0.25, 2102),
        (-3.0, 2100),
        (4.0, 2095),
        (0.75, 2067),
        (-0.75, 2059),
        (-4.0, 2058),
        (-0.25, 2056),
        (f32::POSITIVE_INFINITY, 2053),
        (0.0, 2034),
        (-0.375, 1985),
        (f32::NEGATIVE_INFINITY, 1982),
    ];
    let sample_median = (f32::NAN, None);
    special_random_primitive_floats_helper::<f32>(
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
        -1.0,
        f64::POSITIVE_INFINITY,
        -1.0,
        f64::POSITIVE_INFINITY,
        0.0,
        -1.0,
        f64::POSITIVE_INFINITY,
        -1.0,
        1.0,
        1.0,
        -1.0,
        -1.0,
        -1.0,
        -1.0,
        -1.0,
        -1.0,
        1.0,
        -1.0,
        -1.0,
        1.0,
        0.5,
        1.0,
        -1.0,
        1.0,
        -1.0,
        1.0,
        1.0,
        0.0,
        -1.0,
        f64::NEGATIVE_INFINITY,
        0.0,
        -1.0,
        1.0,
        -1.0,
        -1.0,
        -1.0,
        -0.0,
        1.0,
        -1.0,
        -0.5,
        -1.0,
        f64::POSITIVE_INFINITY,
        f64::NEGATIVE_INFINITY,
        f64::POSITIVE_INFINITY,
        1.0,
    ];
    let common_values = &[
        (1.0, 358273),
        (-1.0, 357906),
        (f64::POSITIVE_INFINITY, 50304),
        (f64::NEGATIVE_INFINITY, 50127),
        (-0.0, 49984),
        (f64::NAN, 49926),
        (0.0, 49868),
        (2.0, 5548),
        (0.5, 5531),
        (1.5, 5489),
        (-1.5, 5469),
        (-2.0, 5383),
        (-0.5, 5355),
        (-4.0, 94),
        (-0.75, 90),
        (-0.25, 89),
        (3.0, 86),
        (-3.0, 80),
        (0.25, 79),
        (4.0, 75),
    ];
    let sample_median = (f64::NAN, None);
    special_random_primitive_floats_helper::<f64>(
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
        1.375,
        2.5,
        f64::POSITIVE_INFINITY,
        -1.0,
        -1.0,
        -2.0,
        -3.5,
        1.5,
        2.0,
        -1.0,
        -3.0,
        -2.0,
        f64::POSITIVE_INFINITY,
        -6.671875,
        -1.75,
        -1.5,
        f64::POSITIVE_INFINITY,
        3.5,
        -0.1875,
        -1.0,
        0.25,
        1.5,
        6.0,
        -6.0,
        6.0,
        -0.5,
        0.125,
        1.0,
        f64::POSITIVE_INFINITY,
        -0.1875,
        -4.0,
        0.0,
        0.875,
        -7.0,
        -6.75,
        -2.5,
        0.125,
        -2.0,
        -0.5,
        -0.875,
        5.0,
        -16.0,
        16.0,
        -3.0,
        1.0,
        f64::POSITIVE_INFINITY,
        -1.0,
        8.0,
        4.0,
    ];
    let common_values = &[
        (1.0, 74871),
        (-1.0, 74815),
        (-0.5, 37712),
        (2.0, 37667),
        (1.5, 37661),
        (-1.5, 37408),
        (0.5, 37381),
        (-2.0, 37312),
        (f64::NEGATIVE_INFINITY, 20096),
        (0.0, 20064),
        (f64::POSITIVE_INFINITY, 20042),
        (-0.0, 20022),
        (f64::NAN, 20000),
        (3.0, 18789),
        (-0.75, 18765),
        (-3.0, 18750),
        (0.75, 18729),
        (-0.25, 18729),
        (-4.0, 18710),
        (4.0, 18693),
    ];
    let sample_median = (f64::NAN, None);
    special_random_primitive_floats_helper::<f64>(
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
        0.917144775390625,
        1.6689300537109375e-6,
        0.013153076171875,
        -0.75,
        -102720.0,
        -0.012674331665039062,
        -0.019363800472092407,
        3.4191773291677237,
        5.6328125,
        -0.000057220458984375,
        -0.2560577392578125,
        -26624.0,
        -61440.0,
        -1.6875,
        -34.0,
        0.4453125,
        -0.0013496266637957888,
        -1318.375,
        0.15625,
        0.001068115234375,
        2.1420419216156006e-8,
        -0.052490234375,
        7208960.0,
        -0.18359375,
        247.5,
        7.0,
        -1048576.0,
        -1.1920928955078125e-7,
        0.2578125,
        -0.004608154296875,
        -46137344.0,
        -4096.0,
        22.3125,
        -6.0,
        -6.02734375,
        -3.3521728515625,
        3.814697265625e-6,
        -0.030975341796875,
        0.00006103515625,
        -0.00003659701906144619,
        0.000202178955078125,
        -45.0,
        0.00732421875,
        0.011749267578125,
        21.310546875,
        26.0,
        -0.19921875,
        -190.625,
        -0.078125,
        -6912.0,
    ];
    let common_values = &[
        (-1.0, 2438),
        (1.0, 2334),
        (0.5, 2199),
        (2.0, 2180),
        (1.5, 2158),
        (-0.5, 2128),
        (-1.5, 2117),
        (-2.0, 2081),
        (f64::POSITIVE_INFINITY, 2053),
        (0.0, 2034),
        (-3.0, 1982),
        (f64::NEGATIVE_INFINITY, 1982),
        (-0.0, 1961),
        (f64::NAN, 1959),
        (-4.0, 1936),
        (3.0, 1931),
        (0.25, 1909),
        (-0.25, 1907),
        (0.75, 1901),
        (4.0, 1880),
    ];
    let sample_median = (f64::NAN, None);
    special_random_primitive_floats_helper::<f64>(
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

fn special_random_primitive_floats_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(special_random_primitive_floats::<T>(
        EXAMPLE_SEED,
        0,
        1,
        10,
        1,
        1,
        10
    ));
    assert_panic!(special_random_primitive_floats::<T>(
        EXAMPLE_SEED,
        1,
        0,
        10,
        1,
        1,
        10
    ));
    assert_panic!(special_random_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        1,
        1,
        10
    ));
    assert_panic!(special_random_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        0,
        1,
        10
    ));
    assert_panic!(special_random_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        10,
        1,
        1,
        0
    ));
    assert_panic!(special_random_primitive_floats::<T>(
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
fn special_random_primitive_floats_fail() {
    apply_fn_to_primitive_floats!(special_random_primitive_floats_fail_helper);
}
