// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_float::test_util::arithmetic::add::{
    add_prec_round_naive, add_rational_prec_round_naive, rug_add, rug_add_rational,
    rug_add_rational_round, rug_add_round,
};
use malachite_float::test_util::common::{
    emulate_primitive_float_fn_2, parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_1, float_float_rounding_mode_triple_gen_var_4,
    float_float_rounding_mode_triple_gen_var_5, float_float_rounding_mode_triple_gen_var_6,
    float_float_rounding_mode_triple_gen_var_7, float_float_rounding_mode_triple_gen_var_8,
    float_float_rounding_mode_triple_gen_var_9,
    float_float_unsigned_rounding_mode_quadruple_gen_var_1, float_float_unsigned_triple_gen_var_1,
    float_gen, float_pair_gen, float_pair_gen_var_2, float_pair_gen_var_3, float_pair_gen_var_4,
    float_pair_gen_var_5, float_pair_gen_var_6, float_pair_gen_var_7, float_rational_pair_gen,
    float_rational_rounding_mode_triple_gen_var_1,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_1,
    float_rational_unsigned_triple_gen_var_1, float_rounding_mode_pair_gen,
    float_unsigned_pair_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_1,
    rational_rounding_mode_pair_gen_var_6, rational_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use malachite_q::Rational;
use std::cmp::{max, Ordering};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_add() {
    let test = |s, s_hex, t, t_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let sum = x.clone() + y.clone();
        assert!(sum.is_valid());

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);

        let sum_alt = x.clone() + &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = &x + y.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = &x + &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));

        let mut sum_alt = x.clone();
        sum_alt += y.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let mut sum_alt = x.clone();
        sum_alt += &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_add(
                rug::Float::exact_from(&x),
                rug::Float::exact_from(&y)
            ))),
            ComparableFloatRef(&sum)
        );

        let sum_alt = add_prec_round_naive(
            x.clone(),
            y.clone(),
            max(x.significant_bits(), y.significant_bits()),
            RoundingMode::Nearest,
        )
        .0;
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    };
    test("NaN", "NaN", "NaN", "NaN", "NaN", "NaN");
    test("NaN", "NaN", "Infinity", "Infinity", "NaN", "NaN");
    test("NaN", "NaN", "-Infinity", "-Infinity", "NaN", "NaN");
    test("NaN", "NaN", "0.0", "0x0.0", "NaN", "NaN");
    test("NaN", "NaN", "-0.0", "-0x0.0", "NaN", "NaN");

    test("Infinity", "Infinity", "NaN", "NaN", "NaN", "NaN");
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", "Infinity", "Infinity",
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", "Infinity", "Infinity",
    );

    test("-Infinity", "-Infinity", "NaN", "NaN", "NaN", "NaN");
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
    );

    test("0.0", "0x0.0", "NaN", "NaN", "NaN", "NaN");
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", "0.0", "0x0.0");
    test("0.0", "0x0.0", "-0.0", "-0x0.0", "0.0", "0x0.0");

    test("-0.0", "-0x0.0", "NaN", "NaN", "NaN", "NaN");
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("-0.0", "-0x0.0", "0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", "-0.0", "-0x0.0");

    test("123.0", "0x7b.0#7", "NaN", "NaN", "NaN", "NaN");
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
    );
    test("123.0", "0x7b.0#7", "0.0", "0x0.0", "123.0", "0x7b.0#7");
    test("123.0", "0x7b.0#7", "-0.0", "-0x0.0", "123.0", "0x7b.0#7");

    test("NaN", "NaN", "123.0", "0x7b.0#7", "NaN", "NaN");
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", "Infinity", "Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
    );
    test("0.0", "0x0.0", "123.0", "0x7b.0#7", "123.0", "0x7b.0#7");
    test("-0.0", "-0x0.0", "123.0", "0x7b.0#7", "123.0", "0x7b.0#7");

    // - in add_float_significands_same_prec_lt_w
    // - x_exp < y_exp in add_float_significands_same_prec_lt_w
    // - exp_diff < shift in add_float_significands_same_prec_lt_w
    // - exp_diff < shift && !overflow in add_float_significands_same_prec_lt_w
    // - (round_bit != 0 || sticky_bit == 0) && rm == Nearest in
    //   add_float_significands_same_prec_lt_w
    // - round_bit != 0 && (sticky_bit != 0 || (sum & shift_bit) != 0) in
    //   add_float_significands_same_prec_lt_w
    // - rm == Nearest && sum == 0 in add_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#1", "2.0", "0x2.0#1", "4.0", "0x4.0#1");
    // - in add_float_significands_same_prec_general
    // - out_bits > exp_diff in add_float_significands_same_prec_general
    // - overlap <= ys_len in add_float_significands_same_prec_general
    // - shift2 != 0 in add_float_significands_same_prec_general
    // - out_len - k <= overlap in add_float_significands_same_prec_general
    // - out_len <= xs_len second time in add_float_significands_same_prec_general
    // - !y in add_float_significands_same_prec_general
    // - round_bit == Uninitialized && shift != 0 in add_float_significands_same_prec_general
    // - shift > 1 in add_float_significands_same_prec_general
    // - x == 0 second time in add_float_significands_same_prec_general
    // - xs_len <= out_len && following_bits != True in add_float_significands_same_prec_general
    // - difw > 0 && difw > ys_len && exp_diff <= out_bits in
    //   add_float_significands_same_prec_general
    // - exp_diff_rem != 0 || yi != 0 second time in add_float_significands_same_prec_general
    // - exp_diff_rem != 0 second time in add_float_significands_same_prec_general
    // - round_bit != Uninitialized seventh time in add_float_significands_same_prec_general
    // - yi == 0 second time in add_float_significands_same_prec_general
    // - in add_float_significands_same_prec_general_round
    // - following_bits == False && round_bit == False in
    //   add_float_significands_same_prec_general_round
    test("1.0", "0x1.0#1", "2.0", "0x2.0#2", "3.0", "0x3.0#2");
    test("1.0", "0x1.0#2", "2.0", "0x2.0#1", "3.0", "0x3.0#2");
    // round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#2", "2.0", "0x2.0#2", "3.0", "0x3.0#2");
    test("1.0", "0x1.000#10", "2.0", "0x2.00#10", "3.0", "0x3.00#10");

    // - exp_diff < shift && overflow in add_float_significands_same_prec_lt_w
    // - round_bit == 0 || (sticky_bit == 0 && (sum & shift_bit) == 0)
    // - in add_float_significands_same_prec_lt_w
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "4.555806215962888",
        "0x4.8e4950f0795fc#53",
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1.727379091216698",
        "0x1.ba35842091e63#53",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-4.555806215962888",
        "-0x4.8e4950f0795fc#53",
    );

    // - x_exp > y_exp in add_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#1", "0.0002", "0x0.001#1", "1.0", "0x1.0#1");
    test("1.0", "0x1.0#1", "-1.0", "-0x1.0#1", "0.0", "0x0.0");
    // - x_exp == y_exp in add_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1", "2.0", "0x2.0#1");
    // - rm == Nearest && sum != 0 in add_float_significands_same_prec_lt_w
    test("1.0", "0x1.0#3", "1.8", "0x1.c#3", "3.0", "0x3.0#3");
    // - exp_diff >= Limb::WIDTH in add_float_significands_same_prec_lt_w
    test(
        "7.7e14",
        "0x2.bcE+12#8",
        "1.237e-9",
        "0x5.50E-8#8",
        "7.7e14",
        "0x2.bcE+12#8",
    );
    // - shift <= exp_diff < Limb::WIDTH in add_float_significands_same_prec_lt_w
    // - shift <= exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_lt_w
    test(
        "1.852193494e22",
        "0x3.ec137baE+18#29",
        "241425944.0",
        "0xe63de18.0#29",
        "1.852193494e22",
        "0x3.ec137baE+18#29",
    );
    // - shift <= exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_lt_w
    test(
        "1.999999999999993",
        "0x1.fffffffffffe#48",
        "5.96046447753906e-8",
        "0x1.000000000000E-6#48",
        "2.00000005960464",
        "0x2.000001000000#48",
    );

    // - in add_float_significands_same_prec_w
    // - x_exp == y_exp in add_float_significands_same_prec_w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0",
        "0x1.0000000000000000#64",
        "2.0",
        "0x2.0000000000000000#64",
    );
    // - x_exp < y_exp in add_float_significands_same_prec_w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_w
    // - !overflow in add_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "2.0",
        "0x2.0000000000000000#64",
        "3.0",
        "0x3.0000000000000000#64",
    );
    // - x_exp > y_exp in add_float_significands_same_prec_w
    test(
        "2.0",
        "0x2.0000000000000000#64",
        "1.0",
        "0x1.0000000000000000#64",
        "3.0",
        "0x3.0000000000000000#64",
    );
    // - (round_bit != 0) || (sticky_bit != 0) && rm == Nearest in
    //   add_float_significands_same_prec_w
    // - round_bit == 0 || (sticky_bit == 0 && (sum & 1) == 0) in add_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        "2.0",
        "0x2.0000000000000000#64",
    );
    // - round_bit != 0 && (sticky_bit != 0 || (sum & 1) != 0) in add_float_significands_same_prec_w
    // - round_bit != 0 && (sticky_bit != 0 || (sum & 1) != 0) and !overflow in
    //   add_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000003",
        "0x1.0000000000000006#64",
        "2.0000000000000000004",
        "0x2.0000000000000008#64",
    );
    // - exp_diff >= Limb::WIDTH in add_float_significands_same_prec_w
    test(
        "5.9376349676904431794e-6",
        "0x0.0000639df2b03f3e49a70#64",
        "2.9347251290514630352e-45",
        "0x1.0c11b075f03d6daeE-37#64",
        "5.9376349676904431794e-6",
        "0x0.0000639df2b03f3e49a70#64",
    );
    // - overflow in add_float_significands_same_prec_w
    test(
        "0.00022185253582909293959",
        "0x0.000e8a1162cbb1a4265#64",
        "0.000029745661521717034001",
        "0x0.0001f30ca4b8117ff0a0#64",
        "0.0002515981973508099736",
        "0x0.00107d1e0783c324170#64",
    );
    // - round_bit != 0 && (sticky_bit != 0 || (sum & 1) != 0) and overflow in
    //   add_float_significands_same_prec_w
    test(
        "63.999999999999999997",
        "0x3f.ffffffffffffffc#64",
        "64.0",
        "0x40.000000000000000#64",
        "128.0",
        "0x80.00000000000000#64",
    );

    // - in add_float_significands_same_prec_gt_w_lt_2w
    // - x_exp == y_exp in add_float_significands_same_prec_gt_w_lt_2w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.0",
        "0x1.0000000000000000#65",
        "2.0",
        "0x2.0000000000000000#65",
    );
    // - x_exp < y_exp in add_float_significands_same_prec_gt_w_lt_2w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_gt_w_lt_2w
    // - exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "2.0",
        "0x2.0000000000000000#65",
        "3.0",
        "0x3.0000000000000000#65",
    );
    // - x_exp > y_exp in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "2.0",
        "0x2.0000000000000000#65",
        "1.0",
        "0x1.0000000000000000#65",
        "3.0",
        "0x3.0000000000000000#65",
    );
    // - round_bit != 0 || sticky_bit != 0 in add_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest in add_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest && (sum_0 != 0 || sum_1 != 0) in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        "2.0",
        "0x2.0000000000000000#65",
    );
    // - rm == Nearest && sum_1 != 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000016",
        "0x1.0000000000000003#65",
        "2.0000000000000000002",
        "0x2.0000000000000004#65",
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_w_lt_2w
    // - Limb::WIDTH < exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_w_lt_2w
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && !overflow in
    //   add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        "18.63123034252626794758647",
        "0x12.a1984fcd64a8ae228eef#85",
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
    );
    // - exp_diff >= Limb::WIDTH * 2 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "4.8545822922649671226e27",
        "0xf.af9dc963a0709f78E+22#65",
        "1.14823551075108882469e-96",
        "0x2.73dea72af3fe6314E-80#65",
        "4.8545822922649671226e27",
        "0xf.af9dc963a0709f78E+22#65",
    );
    // - exp_diff == Limb::WIDTH in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "19585.2851423168986928116147584507795",
        "0x4c81.48ff163dc91a0d4bd90309b0f8#116",
        "372369974082165972902790.766638151683",
        "0x4eda377c7f0d747fa386.c44265dd58#116",
        "372369974082165972922376.05178046858",
        "0x4eda377c7f0d747ff008.0d417c1b20#116",
    );
    // - exp_diff < Limb::WIDTH && overflow in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "18.492649216180435830000529",
        "0x12.7e1e424fe51f1bb914c0#85",
        "56.637589789906471403844847",
        "0x38.a339159fe96c1722fdfe#85",
        "75.130239006086907233845378",
        "0x4b.215757efce8b32dc12c0#85",
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && overflow in
    //   add_float_significands_same_prec_gt_w_lt_2w
    test(
        "5.29395592276605355108231857701752e-23",
        "0x4.00000007e000fffffff0000000E-19#107",
        "255.999999999999999999999947060441",
        "0xff.ffffffffffffffffffc000000#107",
        "256.0",
        "0x100.0000000000000000000000000#107",
    );
    // - rm == Nearest && sum_1 == 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "0.0000610351562499999999996",
        "0x0.0003ffffffffffffffff8#67",
        "17179869183.9999389648",
        "0x3ffffffff.fffc00000#67",
        "17179869184.0",
        "0x400000000.00000000#67",
    );

    // - in add_float_significands_same_prec_2w
    // - x_exp == y_exp in add_float_significands_same_prec_2w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "2.0",
        "0x2.00000000000000000000000000000000#128",
    );
    // - x_exp < y_exp in add_float_significands_same_prec_2w
    // - exp_diff < TWICE_WIDTH in add_float_significands_same_prec_2w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_2w
    // - exp_diff < TWICE_WIDTH && !overflow in add_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "2.0",
        "0x2.00000000000000000000000000000000#128",
        "3.0",
        "0x3.00000000000000000000000000000000#128",
    );
    // - x_exp > y_exp in add_float_significands_same_prec_2w
    test(
        "2.0",
        "0x2.00000000000000000000000000000000#128",
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "3.0",
        "0x3.00000000000000000000000000000000#128",
    );
    // - round_bit != 0 || sticky_bit != 0 in add_float_significands_same_prec_2w
    // - rm == Nearest in add_float_significands_same_prec_2w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (sum_0 & 1) == 0)) in
    //   add_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        "2.0",
        "0x2.00000000000000000000000000000000#128",
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (sum_0 & 1) != 0) in
    //   add_float_significands_same_prec_2w
    // - rm == Nearest && !overflow in add_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000018",
        "0x1.00000000000000000000000000000006#128",
        "2.00000000000000000000000000000000000002",
        "0x2.00000000000000000000000000000008#128",
    );
    // - exp_diff >= TWICE_WIDTH in add_float_significands_same_prec_2w
    // - exp_diff > TWICE_WIDTH in add_float_significands_same_prec_2w
    test(
        "5.80991149045382428948889299639419733262e-6",
        "0x0.00006179613d776a1c835894818a219f488e8#128",
        "5.07801249136957145270807726205511855421e-45",
        "0x1.cfd8608b7c32de2bbcfecf8bcf8a2d00E-37#128",
        "5.80991149045382428948889299639419733262e-6",
        "0x0.00006179613d776a1c835894818a219f488e8#128",
    );
    // - Limb::WIDTH <= exp_diff < TWICE_WIDTH in add_float_significands_same_prec_2w
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH in add_float_significands_same_prec_2w
    test(
        "4354249796990942.35435357526597783143164",
        "0xf782ac869b7de.5ab6ea78fcf0cc5079f#128",
        "8.03239453825726512240307053405256016022e-10",
        "0x3.732bce7aa121827a284545a25f32dc68E-8#128",
        "4354249796990942.35435357606921728525736",
        "0xf782ac869b7de.5ab6ea7c701c9acb1b1#128",
    );
    // - exp_diff == TWICE_WIDTH in add_float_significands_same_prec_2w
    test(
        "15732412727332569995335732833027757624.44",
        "0xbd5f3d586bc01069a1d94f5ab5a1638.7#128",
        "0.0373708302820085888760745841639896128921",
        "0x0.0991227de2b63edc67164401ce8ebdb04#128",
        "15732412727332569995335732833027757624.5",
        "0xbd5f3d586bc01069a1d94f5ab5a1638.8#128",
    );
    // - Limb::WIDTH == exp_diff in add_float_significands_same_prec_2w
    test(
        "1.057437459917463716438672572710788562701e-17",
        "0xc.310127aae1df1a1cb12f60c4d339d76E-15#128",
        "148.0549133677002965445211858794413066474",
        "0x94.0e0ecd6e62d0a8c7c7c2a633277e3e#128",
        "148.054913367700296555095560478615943812",
        "0x94.0e0ecd6e62d0a98ad7d520e1456fe0#128",
    );
    // - exp_diff < TWICE_WIDTH && overflow in add_float_significands_same_prec_2w
    test(
        "990.890284854484258981204316304960898664",
        "0x3de.e3e9b54e224e900a8701c94cea27bc#128",
        "111.972242543885876168914754084523121772",
        "0x6f.f8e4e329c509f04b7f9497ec8ce6438#128",
        "1102.862527398370135150119070389484020437",
        "0x44e.dcce9877e758805606966139770e00#128",
    );
    // - rm == Nearest && overflow in add_float_significands_same_prec_2w
    test(
        "1152920954851033088.0",
        "0xfffff8000000000.00000000000000000#128",
        "549755813887.999999999999999999999999998",
        "0x7fffffffff.ffffffffffffffffffffff8#128",
        "1152921504606846976.0",
        "0x1000000000000000.00000000000000000#128",
    );

    // - in add_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_gt_2w_lt_3w
    // - round_bit != 0 || sticky_bit != 0 in add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && (round_bit == 0 || (sticky_bit == 0
    //   && (sum_0 & shift_bit) == 0)) in add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && (round_bit == 0 || (sticky_bit == 0
    //   && (sum_0 & shift_bit) == 0)) && sum != 0 in add_float_significands_same_prec_gt_2w_lt_3w
    // - x_exp > y_exp in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.0",
        "0x2.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "3.0",
        "0x3.00000000000000000000000000000000#129",
    );
    // - x_exp < y_exp in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "2.0",
        "0x2.00000000000000000000000000000000#129",
        "3.0",
        "0x3.00000000000000000000000000000000#129",
    );
    // - x_exp == y_exp in add_float_significands_same_prec_gt_2w_lt_3w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "2.0",
        "0x2.00000000000000000000000000000000#129",
    );
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && round_bit != 0 && (sticky_bit != 0
    //   || (sum_0 & shift_bit) != 0) in add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && round_bit != 0 && (sticky_bit != 0
    //   || (sum_0 & shift_bit) != 0) && sum != 0 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000009",
        "0x1.00000000000000000000000000000003#129",
        "2.000000000000000000000000000000000000012",
        "0x2.00000000000000000000000000000004#129",
    );
    // - Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH * 2 < exp_diff < Limb::WIDTH * 3 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3 && !overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.024076700393272432111968987625898501371897741e-29",
        "0x1.9a88122864b9c4b577e4b655958954f82345dE-24#149",
        "245906107849378561117126906.9059035528266331265",
        "0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
        "245906107849378561117126906.9059035528266331265",
        "0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
    );
    // - exp_diff >= Limb::WIDTH * 3 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.397610888919711045634814958598336677777534377e47",
        "0x4.d0791b9428a6b4fc52e44e537ab5a0f269ad60E+39#155",
        "6.8892360159362421595728818935378487832685754059e-50",
        "0x1.9c693c182df3035eef00d41638bbdd942f4d498E-41#155",
        "4.397610888919711045634814958598336677777534377e47",
        "0x4.d0791b9428a6b4fc52e44e537ab5a0f269ad60E+39#155",
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH < exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && !overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.2850537238606374652351877988811796373898773e-22",
        "0x2.0607fd4819748c532aad3528693c1e3c1966E-18#146",
        "978.49328809934495391839880801989439981236569",
        "0x3d2.7e4820fe314caadcb9a156bef2f1c8e53c#146",
        "978.49328809934495391839923652526678587611222",
        "0x3d2.7e4820fe314caadcbba75ebc3b0b3d718f#146",
    );
    // - exp_diff < Limb::WIDTH && overflow in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4499918.46219262840948309077470961400786337",
        "0x44a9ce.7652418f789422bc22220831e2030#137",
        "64560208.0262619516023687759351781439347886",
        "0x3d91c50.06b91a6f42e5205070f82f89eefa#137",
        "69060126.488454580011851866709887757942652",
        "0x41dc61e.7d0b5bfebb79430c931a37bbd0fc#137",
    );
    // - exp_diff == Limb::WIDTH * 2 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "563971925627753843356041629019151473018178607215.42",
        "0x62c960337e963a378ba6626ea422d8a5e623986f.6c#165",
        "1301375421.83361702516620516356439489325145225661938",
        "0x4d9169bd.d567ece47a47ef60371d48c969ba8765d4#165",
        "563971925627753843356041629019151473019479982637.25",
        "0x62c960337e963a378ba6626ea422d8a633b5022d.40#165",
    );
    // - exp_diff == Limb::WIDTH in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "226.9305090753243797994707628568605406194",
        "0xe2.ee35d7bf263fda8c632644ad7c49d98#130",
        "4262448175090788889452.984188256984861391",
        "0xe71159efd3a67e736c.fbf3c2f8db72fb8#130",
        "4262448175090788889679.91469733230924119",
        "0xe71159efd3a67e744f.ea299ab801b2d60#130",
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1180591620717411303423.999999999999999999999999997",
        "0x3fffffffffffffffff.ffffffffffffffffffffff#158",
        "5.68434188616351954822429632036496545806324230121e-14",
        "0x1.000000000fffffffffffe0000000000000000038E-11#158",
        "1180591620717411303424.00000000000005684341886163",
        "0x400000000000000000.00000000001000000000fe#158",
    );
    // - Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3 && overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && round_bit != 0 && (sticky_bit != 0
    //   || (sum_0 & shift_bit) != 0) && sum == 0
    test(
        "4503599627370495.9999999999999999999999999996",
        "0xfffffffffffff.ffffffffffffffffffffffe#143",
        "3.3087224509824797385046520537834728287650668e-24",
        "0x4.00000003ffff0000000000000000fffffffE-20#143",
        "4503599627370496.000000000000000000000003309",
        "0x10000000000000.00000000000000000004000#143",
    );

    // - in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && round_bit == 0 in add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
    );
    // - x_exp < y_exp in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec in add_float_significands_same_prec_ge_3w
    // - in add_significands_rsh_to_out
    // - exp_diff < Limb::WIDTH in add_significands_rsh_to_out
    // - 0 < exp_diff < prec && shift == 0 in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && limb == 0 in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && rm == Nearest in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && rm == Nearest && round_bit == 0 in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "3.0",
        "0x3.000000000000000000000000000000000000000000000000#192",
    );
    // - x_exp > y_exp in add_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "3.0",
        "0x3.000000000000000000000000000000000000000000000000#192",
    );
    // - 0 < exp_diff < prec && shift != 0 in add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#193",
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#193",
        "3.0",
        "0x3.000000000000000000000000000000000000000000000000#193",
    );
    // - exp_diff == 0 && rm == Nearest in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && rm == Nearest && out[0] & shift_bit == 0 in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
    );
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && sticky_bit == 0 && out[0] &
    //   shift_bit == 0
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        "3.0",
        "0x3.000000000000000000000000000000000000000000000000#192",
    );
    // - exp_diff == 0 && rm == Nearest && out[0] & shift_bit != 0 in
    //   add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && rm == Nearest && out[0] & shift_bit != 0 && !carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.000000000000000000000000000000000000000000000000000000001",
        "0x1.000000000000000000000000000000000000000000000006#192",
        "2.0000000000000000000000000000000000000000000000000000000013",
        "0x2.000000000000000000000000000000000000000000000008#192",
    );
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] &
    //   shift_bit != 0) in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] &
    //   shift_bit != 0) && !carry in add_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.000000000000000000000000000000000000000000000000000000001",
        "0x1.000000000000000000000000000000000000000000000006#192",
        "3.0000000000000000000000000000000000000000000000000000000013",
        "0x3.000000000000000000000000000000000000000000000008#192",
    );
    // - exp_diff >= Limb::WIDTH in add_significands_rsh_to_out
    // - exp_diff >= Limb::WIDTH && r != 0 in add_significands_rsh_to_out
    test(
        "7.28057116938384227432903448367767196428679514765398378973101e-48",
        "0xa.a3fc2da1f20fb2d9771f86d3c16a444cd62d5d139e3935f24E-40#198",
        "3.5123473778825578958968695187657587760357139395948269588971e-27",
        "0x1.1646de419a6dbd3466f3081403a87d719b7a765a1ec69e4658E-22#198",
        "3.5123473778825578959041500899351426183100429740785046308614e-27",
        "0x1.1646de419a6dbd3471970441a59a8d2474f195e0f288088aa8E-22#198",
    );
    // - exp_diff >= prec in add_float_significands_same_prec_ge_3w
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w && (rm == Nearest || rm == Floor
    //   || rm == Down)
    test(
        "4.1322282880219162156901559575161649173615955518072607291207e86",
        "0xd.4b575f05941ee41ef3ef9a37068d9d453f22eb3bf80bd1b0E+71#193",
        "0.023991386767031193042066748710708351501952890752924613005724",
        "0x0.06244cad8cd272134e34b325815ad281733f2c06231a0ee744#193",
        "4.1322282880219162156901559575161649173615955518072607291207e86",
        "0xd.4b575f05941ee41ef3ef9a37068d9d453f22eb3bf80bd1b0E+71#193",
    );
    // - 0 < exp_diff < prec && limb != 0 in add_float_significands_same_prec_ge_3w
    test(
        "8.699772042374378140693728074838279708562673799416097107796",
        "0x8.b32442b4a730454d66b1b2bdf7a2863d417e6ff22d7f6c58#193",
        "7.5897463681962395437740598844462353563682906392115908083148",
        "0x7.96f99e34566e7be1960d023e431dc5e0a7ad24ad691a1ac4#193",
        "16.289518410570617684467787959284515064930964438627687916112",
        "0x10.4a1de0e8fd9ec12efcbeb4fc3ac04c1de92b949f9699872#193",
    );
    // - exp_diff >= Limb::WIDTH && r == 0 in add_significands_rsh_to_out
    test(
        "6.442552350746554109885349691592991892989624685631192235549e-6",
        "0x0.00006c168d38e231899f0fc85d1888549d5177bdceaee72e15060#192",
        "1476808010161862576835936576709144.7975622615653024045505082",
        "0x48cff00a780a50d34bb694ada218.cc2d0a55f25f9f9126258#192",
        "1476808010161862576835936576709144.797568704117653151104618",
        "0x48cff00a780a50d34bb694ada218.cc2d766c7f9881c2afc48#192",
    );
    // - exp_diff == prec in add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest in add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest && power && !carry in
    //   add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest && !power in add_float_significands_same_prec_ge_3w
    test(
        "4.0635838402455207229400698207668893925379768151364313942222e-23",
        "0x3.1202ecf10ff40b477337957dede18bd7b746884ec977474eE-19#194",
        "1174582238252884689829665592721065057.76655867827770290150723",
        "0xe237601fa3ed6d89b0ae33e924c461.c43d3085aaefab6b5d4#194",
        "1174582238252884689829665592721065057.76655867827770290150729",
        "0xe237601fa3ed6d89b0ae33e924c461.c43d3085aaefab6b5d8#194",
    );
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] &
    //   shift_bit != 0) && carry in add_float_significands_same_prec_ge_3w
    test(
        "4.336808689942017736029811203479766845699938816177735095446e-19",
        "0x7.fffffffffffffffffffffffffffffffe0000000000000000E-16#192",
        "5192296858534827628530496329220095.999999999999999999566319",
        "0xffffffffffffffffffffffffffff.fffffffffffffff80000#192",
        "5192296858534827628530496329220096.0",
        "0x10000000000000000000000000000.00000000000000000000#192",
    );
    // - exp_diff == prec && rm == Nearest && power && carry in
    //   add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest && !power && carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "158456325028528675187087900671.99999999999999999999999999997",
        "0x1ffffffffffffffffffffffff.fffffffffffffffffffffffe#192",
        "2.5243548967072377773175314089049159349542605923488736152645e-29",
        "0x1.fffffffffffffffffffffffffffffffffffffffffffffffeE-24#192",
        "158456325028528675187087900672.0",
        "0x2000000000000000000000000.000000000000000000000000#192",
    );
    // - exp_diff == prec && rm == Nearest && power in add_float_significands_same_prec_ge_3w
    test(
        "332306998926888516295359133097394175.99999997019767761230469",
        "0x3ffffffff0007fffffffffffffffff.ffffff8000000000000#192",
        "2.6469779601696885595885078146238811314105987548828125e-23",
        "0x2.000000000000000000000000000000000000000000000000E-19#192",
        "332306998926888516295359133097394175.99999997019767761230469",
        "0x3ffffffff0007fffffffffffffffff.ffffff8000000000000#192",
    );

    // - shift2 == 0 in add_float_significands_same_prec_general
    // - y in add_float_significands_same_prec_general
    // - shift != 0 in add_float_significands_same_prec_general
    // - x == 0 first time in add_float_significands_same_prec_general
    // - shift == 0 || following_bits != Uninitialized in add_float_significands_same_prec_general
    // - round_bit != Uninitialized || shift == 0 in add_float_significands_same_prec_general
    // - exp_diff_rem == 0 && yi == 0 second time in add_float_significands_same_prec_general
    // - round_bit != Uninitialized sixth time in add_float_significands_same_prec_general
    test("1.0", "0x1.0#1", "1.0", "0x1.0#2", "2.0", "0x2.0#2");
    // - following_bits != False || round_bit != False in
    //   add_float_significands_same_prec_general_round
    // - rm == Nearest in add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits == False in add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits == False && out[0] & shift_bit == 0 in
    //   add_float_significands_same_prec_general_round
    test("1.0", "0x1.0#1", "1.5", "0x1.8#2", "2.0", "0x2.0#2");
    // - rm == Nearest && following_bits == False && out[0] & shift_bit != 0 in
    //   add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits == False && out[0] & shift_bit != 0 && carry in
    //   add_float_significands_same_prec_general_round
    test("2.0", "0x2.0#1", "1.5", "0x1.8#2", "4.0", "0x4.0#2");
    // - rm == Nearest && following_bits == False && out[0] & shift_bit != 0 && !carry in
    //   add_float_significands_same_prec_general_round
    test("1.0", "0x1.0#1", "1.8", "0x1.c#3", "3.0", "0x3.0#3");
    // - x != 0 && x != mask second time in add_float_significands_same_prec_general
    // - rm == Nearest && following_bits != False && round_bit != False in
    //   add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits != False && round_bit != False && !carry in
    //   add_float_significands_same_prec_general_round
    test("1.5", "0x1.8#2", "4.0", "0x4.0#1", "6.0", "0x6.0#2");
    // - rm == Nearest && following_bits != False && round_bit == False in
    //   add_float_significands_same_prec_general_round
    test("4.0", "0x4.0#1", "1.2", "0x1.4#3", "5.0", "0x5.0#3");
    // - x != 0 && x != mask first time in add_float_significands_same_prec_general
    // - shift != 0 && following_bits == Uninitialized in add_float_significands_same_prec_general
    test("1.2", "0x1.4#3", "3.0", "0x3.0#2", "4.0", "0x4.0#3");
    // - rm == Nearest && following_bits != False && round_bit != False && carry in
    //   add_float_significands_same_prec_general_round
    test("1.8", "0x1.c#3", "6.0", "0x6.0#2", "8.0", "0x8.0#3");
    // - out_bits <= exp_diff in add_float_significands_same_prec_general
    // - out_len <= xs_len first time in add_float_significands_same_prec_general
    // - difw > 0 && difw > ys_len && exp_diff > out_bits in
    //   add_float_significands_same_prec_general
    // - round_bit != Uninitialized fifth time in add_float_significands_same_prec_general
    test(
        "8.82188e11",
        "0xc.d668E+9#18",
        "9.75459983374e122",
        "0x1.79c17f063aE+102#40",
        "9.75459983374e122",
        "0x1.79c17f063aE+102#40",
    );
    // - out_len > xs_len first time in add_float_significands_same_prec_general
    test(
        "2.8577648979177105962332201291018926848163080599637e-19",
        "0x5.458a93bffa7b1c05bdd1c0552b60196746d9083cE-16#162",
        "3.569720699507868e50",
        "0xf.4400d3acf388E+41#51",
        "3.5697206995078675404584127554321345196383736430592e50",
        "0xf.4400d3acf3880000000000000000000000000000E+41#162",
    );
    // - overlap > ys_len in add_float_significands_same_prec_general
    // - out_len - k > overlap in add_float_significands_same_prec_general
    // - difw <= 0 || difw <= ys_len in add_float_significands_same_prec_general
    // - round_bit != Uninitialized fourth time in add_float_significands_same_prec_general
    test(
        "29780282551.762684458936866363165",
        "0x6ef0b0cb7.c33f49e84d21bb6040#104",
        "0.00003945598947538",
        "0x0.000295f62f36adb#46",
        "29780282551.762723914926341743141",
        "0x6ef0b0cb7.c341dfde7c58691040#104",
    );
    // - out_len > xs_len second time in add_float_significands_same_prec_general
    test(
        "1.07183972513958531257713938927815e-11",
        "0xb.c8f5eafa12eb9821601f1dd6aeE-10#107",
        "0.00374222828352849",
        "0x0.00f5402c178824#46",
        "0.00374222829424688982311482391965285",
        "0x0.00f5402c235119eafa12eb9821602#107",
    );
    // - exp_diff_rem == 0 second time in add_float_significands_same_prec_general
    test(
        "5.19192095203e-15",
        "0x1.761e097c5E-12#37",
        "7.4e4",
        "0x1.2E+4#5",
        "73728.0",
        "0x12000.00000#37",
    );
    // - shift <= 1 in add_float_significands_same_prec_general
    test(
        "15135.895602865542606017656527713819177465060416097749360065",
        "0x3b1f.e5463ab9b599ce49b83c7988b324dc93ce50b2ed51a18#191",
        "3.581529624499970047886732225242180736649e-8",
        "0x9.9d355ad2b99a587727da095fa3226bf0E-7#130",
        "15135.895602901357902262656228192686499717482223464235884113",
        "0x3b1f.e5463b5388ef7b7551e200fb30c5728e007771ed51a18#191",
    );
    // - round_bit == Uninitialized fourth time in add_float_significands_same_prec_general
    // - round_bit == Uninitialized seventh time in add_float_significands_same_prec_general
    test(
        "8.63643735016344467819174862798332593462e-6",
        "0x0.000090e5374a001358c6606f968bf3813ad9#128",
        "1.84904e-8",
        "0x4.f6a6E-7#18",
        "8.65492771851100147411665059026359937212e-6",
        "0x0.00009134a1aa001358c6606f968bf3813ad9#128",
    );
    // - round_bit == Uninitialized fifth time in add_float_significands_same_prec_general
    test(
        "2.389545997e25",
        "0x1.3c40f7bE+21#29",
        "0.078263259824284000402",
        "0x0.14090f9d6c745bc06#64",
        "2.389545996756557709e25",
        "0x1.3c40f7b000000000E+21#64",
    );
    // - round_bit == Uninitialized seventh time in add_float_significands_same_prec_general
    test(
        "5.7505515877842013577e-7",
        "0x9.a5d7d56cabed47dE-6#64",
        "1.1758894e-14",
        "0x3.4f515E-12#22",
        "5.7505517053731436845e-7",
        "0x9.a5d7d8bbfd3d47dE-6#64",
    );
    // - x != 0 && x == mask second time in add_float_significands_same_prec_general
    // - xs_len <= out_len && following_bits == True in add_float_significands_same_prec_general
    test(
        "1.081090215247020702e-18",
        "0x1.3f14ddfe22c0634E-15#59",
        "6.3799657596147e-8",
        "0x1.12047722d26cE-6#47",
        "6.37996575972280156e-8",
        "0x1.12047722e65d4dcE-6#59",
    );
    // - shift == 0 in add_float_significands_same_prec_general
    test(
        "4.3055451539258443718732375731462554408177909736937057433067e-16",
        "0x1.f06543668e6018c20c17efed72ff6d3d65a4c5dc9db475b0E-13#192",
        "1.6388436e-15",
        "0x7.61754E-13#21",
        "2.0693980969049410047184094732421002104686218250305033868307e-15",
        "0x9.51da83668e6018c20c17efed72ff6d3d65a4c5dc9db475bE-13#192",
    );
    // - yi != 0 second time in add_float_significands_same_prec_general
    test(
        "2.24181435676546e-16",
        "0x1.0276ae5de1e8E-13#47",
        "7.6430700039878539638425372386462404393e-36",
        "0xa.28cd4cb186f5925ddb0d1ecb9681103E-30#128",
        "2.24181435676546206911083333246297446029e-16",
        "0x1.0276ae5de1e80000a28cd4cb186f5926E-13#128",
    );
    // - x != 0 && x == mask first time in add_float_significands_same_prec_general
    test(
        "2.1474796e9",
        "0x7.ffff00E+7#24",
        "8191.9998788833609",
        "0x1fff.fff80fffff0#54",
        "2147487743.9998789",
        "0x80000fff.fff810#54",
    );
}

#[test]
fn test_add_prec() {
    let test = |s, s_hex, t, t_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (sum, o) = x.clone().add_prec(y.clone(), prec);
        assert!(sum.is_valid());

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        let (sum_alt, o_alt) = x.clone().add_prec_val_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = x.add_prec_ref_val(y.clone(), prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = x.add_prec_ref_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_prec_assign(y.clone(), prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_prec_assign_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) =
            add_prec_round_naive(x.clone(), y.clone(), prec, RoundingMode::Nearest);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    };
    test("NaN", "NaN", "NaN", "NaN", 1, "NaN", "NaN", Ordering::Equal);
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        1,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        2,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        2,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        2,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        2,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        2,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "4.555",
        "0x4.8e#10",
        Ordering::Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-2.0",
        "-0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-1.727",
        "-0x1.ba0#10",
        Ordering::Greater,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "1.727",
        "0x1.ba0#10",
        Ordering::Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-4.0",
        "-0x4.0#1",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-4.555",
        "-0x4.8e#10",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        1,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        20,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        1,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        10,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    // - xs_len > out_len in add_float_significands_same_prec_general
    // - following_bits != False || difw > 0 in add_float_significands_same_prec_general
    // - difw <= ys_len in add_float_significands_same_prec_general
    // - exp_diff_rem != 0 || yi != 0 first time in add_float_significands_same_prec_general
    // - yi >= 0 && yi == ys_len in add_float_significands_same_prec_general
    // - following_bits != Uninitialized first time in add_float_significands_same_prec_general
    // - exp_diff_rem != 0 first time in add_float_significands_same_prec_general
    // - yi >= 0 second time in add_float_significands_same_prec_general
    // - yi >= 0 third time in add_float_significands_same_prec_general
    // - x >= y in add_float_significands_same_prec_general
    // - following_bits != False || x == 0 in add_float_significands_same_prec_general
    // - following_bits != False && x != Limb::MAX second time in
    //   add_float_significands_same_prec_general
    test(
        "1.73414747294406e-17",
        "0x1.3fe4cc8cf520E-14#48",
        "5095194424.1679374580403939884785489",
        "0x12fb27f38.2afdf3020e8eaac84a7ec#116",
        62,
        "5095194424.167937458",
        "0x12fb27f38.2afdf300#62",
        Ordering::Less,
    );
    // - yi < 0 || yi != ys_len in add_float_significands_same_prec_general
    // - yi < 0 third time in add_float_significands_same_prec_general
    test(
        "102490490.309858195",
        "0x61be17a.4f52dde#54",
        "140024642456267714260687682888.7395345704093208802906032076327334",
        "0x1c471ab65a12bae8410f87d48.bd52233a2b6095a9fa45bd79bbfc#209",
        126,
        "140024642456267714260790173379.049392765",
        "0x1c471ab65a12bae8417145ec3.0ca50118#126",
        Ordering::Less,
    );
    // - following_bits != 0 && following_bits != Limb::MAX in
    //   add_float_significands_same_prec_general
    // - x != Limb::MAX first time in add_float_significands_same_prec_general
    test(
        "3.6587e-6",
        "0x0.00003d62#14",
        "2.26998937985590408943624715555e46",
        "0x3.f9e6004fa97f534f98cad274E+38#96",
        61,
        "2.269989379855904089e46",
        "0x3.f9e6004fa97f534E+38#61",
        Ordering::Less,
    );
    // - following_bits == Uninitialized first time in add_float_significands_same_prec_general
    // - exp_diff_rem != 0 in add_float_significands_same_prec_general
    // - yi < 0 first time in add_float_significands_same_prec_general
    // - x >= y or not other conditions in add_float_significands_same_prec_general
    // - round_bit == Limb::MAX second time in add_float_significands_same_prec_general
    // - following_bits != False && x != Limb::MAX first time in
    //   add_float_significands_same_prec_general
    test(
        "6058.05208272591415306446968882359605946955168456454",
        "0x17aa.0d554b247ce1b6ab28ba39c8d5992a74c7ac91a#169",
        "0.000144566892208323",
        "0x0.0009796e12f9784#47",
        64,
        "6058.0522272928063612",
        "0x17aa.0d5ec4928fdb2#64",
        Ordering::Less,
    );
    // - following_bits == True in add_float_significands_same_prec_general
    // - round_bit == Uninitialized first time in add_float_significands_same_prec_general
    // - x != Limb::MAX second time in add_float_significands_same_prec_general
    test(
        "3.6596517369110659089355442395654891585e48",
        "0x2.810875a0ca3206afd8c6cf841941830E+40#123",
        "1545.699550397407201099813420288295",
        "0x609.b315bc1ec48a143a74bd53048#109",
        64,
        "3.6596517369110659089e48",
        "0x2.810875a0ca3206b0E+40#64",
        Ordering::Greater,
    );
    // - yi >= 0 first time in add_float_significands_same_prec_general
    test(
        "2.80915429604669102593383052436808885401854724410738829e-11",
        "0x1.ee310ffa09a06a6361f52c2cd8a9569a780b775dc213E-9#177",
        "519241072363118296470.333928838103121952666621563036",
        "0x1c25eadc41d4907d96.557c5c3ed81cab65dab0cf920#166",
        64,
        "5.1924107236311829648e20",
        "0x1.c25eadc41d4907daE+17#64",
        Ordering::Greater,
    );
    // - round_bit != Uninitialized second time in add_float_significands_same_prec_general
    test(
        "559935046210054011882951826578284118061013900.5853448",
        "0x191bbd3588c78488c2f4d122814d5fb34edb8c.95d928#170",
        "3.027932e11",
        "0x4.67fe2E+9#22",
        63,
        "5.599350462100540119e44",
        "0x1.91bbd3588c78488cE+37#63",
        Ordering::Less,
    );
    // - following_bits == False && difw <= 0 in add_float_significands_same_prec_general
    test(
        "7184368264698708563285024670194469326968686224.86386349506591",
        "0x1422880c600dc4fd90a02f1814859aafd658690.dd2628738430#198",
        "1.0296060328202e-24",
        "0x1.3ea5cb49bdaE-20#44",
        61,
        "7.184368264698708565e45",
        "0x1.422880c600dc4feE+38#61",
        Ordering::Greater,
    );
    // - following_bits == False && x != 0 in add_float_significands_same_prec_general
    test(
        "19710666.821984898059985706849",
        "0x12cc2ca.d26d9a2ef9396c5108#94",
        "7.0e4",
        "0x1.0E+4#2",
        61,
        "19776202.82198489807",
        "0x12dc2ca.d26d9a2f0#61",
        Ordering::Greater,
    );
    // - round_bit != Uninitialized first time in add_float_significands_same_prec_general
    test(
        "2.3370796820961060045359802932823709e39",
        "0x6.de392c9978b4267553b428865de8E+32#112",
        "1.187719715482312494e-58",
        "0xb.edbf4827e1e28aaE-49#64",
        63,
        "2.3370796820961060044e39",
        "0x6.de392c9978b4267E+32#63",
        Ordering::Less,
    );
    // - difw > ys_len in add_float_significands_same_prec_general
    // - difw > ys_len || goto_c_read in add_float_significands_same_prec_general
    // - following_bits != Uninitialized second time in add_float_significands_same_prec_general
    // - following_bits == False second time in add_float_significands_same_prec_general
    // - xs[xi] != 0 in add_float_significands_same_prec_general
    test(
        "1248577957.9617995883835430866672787859939813175787209064549678049868",
        "0x4a6bc9a5.f6387f7169e05140ece8db047baba25ac8c576b8fed10fa4#221",
        "1.65314121012e-6",
        "0x0.00001bbc2ffb9e#36",
        126,
        "1248577957.96180124152475320394882420796",
        "0x4a6bc9a5.f6389b2d99dbef40ece8db04#126",
        Ordering::Less,
    );
    // - x < y in add_float_significands_same_prec_general
    // - following_bits == False first time in add_float_significands_same_prec_general
    // - round_bit != False or not other condition in add_float_significands_same_prec_general
    test(
        "1.85445e-25",
        "0x3.9648E-21#15",
        "1.2975739042614492272769355049909712560463719657882671587557999636387971206e-6",
        "0x0.000015c509987b7b0dbb1bf2aae59a4afde515d3ec2c3af539738e362659e1f1b0#243",
        62,
        "1.2975739042614492276e-6",
        "0x0.000015c509987b7b0dbb8#62",
        Ordering::Greater,
    );
    // - following_bits == Uninitialized second time in add_float_significands_same_prec_general
    // - round_bit == Uninitialized third time in add_float_significands_same_prec_general
    // - following_bits != False second time in add_float_significands_same_prec_general
    test(
        "1.3111820218254081035114504135472568116036464005e-6",
        "0x0.000015ff7be10e865ada82cd25acef5baa9c89c25f4#152",
        "2.51465891601e-20",
        "0x7.6c05c64a8E-17#38",
        128,
        "1.311182021825433250100610518712675662596e-6",
        "0x0.000015ff7be10e86d19adf31cdacef5baa9c8#128",
        Ordering::Less,
    );
    // - following_bits == False || x == Limb::MAX second time in
    //   add_float_significands_same_prec_general
    // - !goto_c_read in add_float_significands_same_prec_general
    // - following_bits != False || yi < 0 in add_float_significands_same_prec_general
    test(
        "7.9999999999995452526491135358810425",
        "0x7.ffffffffff800000000000000000#114",
        "0.0039062797764",
        "0x0.0100007fe38#34",
        49,
        "8.00390627977595",
        "0x8.0100007fe300#49",
        Ordering::Equal,
    );
    // - following_bits != False first time in add_float_significands_same_prec_general
    test(
        "2.89901505570589585435e-11",
        "0x1.fe0007ffc00000000E-9#67",
        "134217728.00000381469725",
        "0x8000000.00003ffffffc#74",
        51,
        "134217728.0000038",
        "0x8000000.000040#51",
        Ordering::Less,
    );
    // - yi < 0 second time in add_float_significands_same_prec_general;
    // - goto_c_read in add_float_significands_same_prec_general
    test(
        "4.4474794e-47",
        "0x4.0ffff0E-39#25",
        "7.523135146670945963453530822847271532226579417449854573285752516755e-37",
        "0xf.fffc000001fffffffffffffffffffffffffffc03fffffffc001ffeE-31#219",
        57,
        "7.52313514711569391e-37",
        "0xf.fffc000411fff0E-31#57",
        Ordering::Greater,
    );
    // - round_bit == False and other condition in add_float_significands_same_prec_general
    test(
        "1048575.9999999999999999965",
        "0xfffff.ffffffffffffffc0#81",
        "1.1102230204892534781399170051165479471979625e-16",
        "0x7.ffffff800000003ffe000001fffffffc0000E-14#145",
        48,
        "1048576.0",
        "0x100000.0000000#48",
        Ordering::Less,
    );
    // - x == Limb::MAX first time in add_float_significands_same_prec_general
    // - xi != 0 in add_float_significands_same_prec_general
    test(
        "3.810971975326539e-6",
        "0x0.00003ff00000000004#52",
        "4.90398573077084434674671048688098938757996518e55",
        "0x1.ffffffffffffffffffffffffffffffffffffeE+46#148",
        21,
        "4.903986e55",
        "0x2.00000E+46#21",
        Ordering::Greater,
    );
    // - xs[xi] == 0 in add_float_significands_same_prec_general
    test(
        "135.998",
        "0x87.ff8#19",
        "8796093087743.998046875",
        "0x8000000ffff.ff80000000000000000000000000000#168",
        114,
        "8796093087879.99609375",
        "0x80000010087.ff0000000000000000#114",
        Ordering::Equal,
    );
    // - following_bits == False && yi >= 0 in add_float_significands_same_prec_general
    // - exp_diff_rem != 0 && y_prec << (Limb::WIDTH - exp_diff_rem) != 0 in
    //   add_float_significands_same_prec_general
    test(
        "4610525002867933183.9999999999998",
        "0x3ffbe00fffffffff.ffffffffffc#104",
        "2.27373688992450493e-13",
        "0x4.000003ffc00000E-11#58",
        47,
        "4.61052500286793e18",
        "0x3.ffbe01000000E+15#47",
        Ordering::Less,
    );
    // - exp_diff_rem == 0 first time in add_float_significands_same_prec_general
    test(
        "199484.9246647061832808582",
        "0x30b3c.ecb6d380d2988d78#79",
        "9.2945767606483e-15",
        "0x2.9dbeadc4568E-12#45",
        63,
        "199484.92466470618328",
        "0x30b3c.ecb6d380d298#63",
        Ordering::Less,
    );
    // - exp_diff_rem == 0 || y_prec << (Limb::WIDTH - exp_diff_rem) == 0 in
    //   add_float_significands_same_prec_general
    // - difw <= ys_len && !goto_c_read in add_float_significands_same_prec_general
    test(
        "128.000244140566792339087",
        "0x80.000fffffc00000000#75",
        "7.5",
        "0x7.800000000000000000000#87",
        60,
        "135.5002441405667923",
        "0x87.800fffffc0000#60",
        Ordering::Equal,
    );
    // - exp_diff_rem == 0 second time in add_float_significands_same_prec_general
    // - yi != 0 first time in add_float_significands_same_prec_general
    test(
        "127.9999999999999997744859485366053840029607161188794193511",
        "0x7f.ffffffffffffefc000007fffffffffff0000000001e000#190",
        "64.0000000000000000000000000000000000000000000000410536659471",
        "0x40.000000000000000000000000000000000000003c000000008#200",
        90,
        "191.9999999999999997744859485",
        "0xbf.ffffffffffffefc000008#90",
        Ordering::Greater,
    );
    // - following_bits == False || x == Limb::MAX first time in
    //   add_float_significands_same_prec_general
    test(
        "1927941831.98168915743",
        "0x72ea0ec7.fb4ffb0a0#65",
        "10702647.0",
        "0xa34f37.0#24",
        63,
        "1938644478.9816891574",
        "0x738d5dfe.fb4ffb0a#63",
        Ordering::Equal,
    );
    // - round_bit == Uninitialized sixth time in add_float_significands_same_prec_general
    test(
        "8.69134330408049e-16",
        "0x3.ea0b2d4be674E-13#48",
        "15492.756587362761273446",
        "0x3c84.c1afb59ba066160#71",
        128,
        "15492.75658736276127431557748582959734014",
        "0x3c84.c1afb59ba06654a0b2d4be6740000#128",
        Ordering::Equal,
    );
    // - ys[yi] != 0 in add_float_significands_same_prec_general
    test(
        "2.384181243542116135358810424804687500000000000000000024885983948e-7",
        "0x3.ffff8000000000000000000000000000000000000003ffc00000E-6#210",
        "1.5",
        "0x1.8000000000000000000#75",
        55,
        "1.50000023841812435",
        "0x1.800003ffff8000#55",
        Ordering::Less,
    );
    // - x == Limb::MAX second time in add_float_significands_same_prec_general
    test(
        "98079714615416886934934209737619787751599303785390800896.0002441405",
        "0x3fffffffffffffffffffffffffffffffffffff800000000.000fffff8#219",
        "5.8207660907e-11",
        "0x3.fffffffe0E-9#35",
        63,
        "9.807971461541688693e55",
        "0x4.000000000000000E+46#63",
        Ordering::Greater,
    );
    // - xi == 0 in add_float_significands_same_prec_general
    test(
        "2.996272864212510369e-95",
        "0x3.fffffff00000000E-79#60",
        "1.49012180372665170580148696899414062499e-8",
        "0x4.0000fffffffffffffffffffffffffff8E-7#128",
        27,
        "1.4901218e-8",
        "0x4.000100E-7#27",
        Ordering::Greater,
    );
    // - x < y and other conditions in add_float_significands_same_prec_general
    test(
        "0.00024414062499999999999999979805160826342",
        "0x0.000ffffffffffffffffffff00000000000#124",
        "1.32348897761965396e-23",
        "0xf.ffffff80000008E-20#57",
        63,
        "0.000244140625",
        "0x0.0010000000000000000#63",
        Ordering::Less,
    );
    // - ys[yi] == 0 in add_float_significands_same_prec_general
    test(
        "0.99951171875",
        "0x0.ffe00000000000000000000#92",
        "0.017578124999996447286321199499070644378662109375",
        "0x0.047fffffffff00000000000000000000000000000000000000000000#217",
        48,
        "1.01708984375",
        "0x1.046000000000#48",
        Ordering::Greater,
    );
    // - yi == 0 first time in add_float_significands_same_prec_general
    test(
        "67108864.000000000000000000027105054312137610850186320021748542784370058",
        "0x4000000.00000000000000007ffffffffffffffffffffffffffffff800000#236",
        "3.63797880709171295166015625e-12",
        "0x4.000000000000000000000E-10#86",
        103,
        "67108864.0000000000036379788342",
        "0x4000000.0000000004000000800#103",
        Ordering::Greater,
    );
    test(
        "3.91923414751995775967065104797896464447978457860112833061347e-17",
        "0x2.d2f8998a5fa796b9de129923cf4d51e04f27e6b6a368fadb0E-14#197",
        "2.75164948336014612968759798085025748008e-17",
        "0x1.fb96f911d37dd9fe4933cbeee25d7ddeE-14#128",
        8,
        "6.68e-17",
        "0x4.d0E-14#8",
        Ordering::Greater,
    );
}

#[test]
fn add_prec_fail() {
    assert_panic!(Float::NAN.add_prec(Float::NAN, 0));
    assert_panic!(Float::NAN.add_prec_val_ref(&Float::NAN, 0));
    assert_panic!(Float::NAN.add_prec_ref_val(Float::NAN, 0));
    assert_panic!(Float::NAN.add_prec_ref_ref(&Float::NAN, 0));
    assert_panic!({
        let mut x = Float::NAN;
        x.add_prec_assign(Float::NAN, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.add_prec_assign_ref(&Float::NAN, 0)
    });
}

#[test]
fn test_add_round() {
    let test = |s, s_hex, t, t_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (sum, o) = x.clone().add_round(y.clone(), rm);
        assert!(sum.is_valid());

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        let (sum_alt, o_alt) = x.clone().add_round_val_ref(&y, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = x.add_round_ref_val(y.clone(), rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = x.add_round_ref_ref(&y, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_round_assign(y.clone(), rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_round_assign_ref(&y, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);
        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sum, rug_o) =
                rug_add_round(rug::Float::exact_from(&x), rug::Float::exact_from(&y), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sum)),
                ComparableFloatRef(&sum),
            );
            assert_eq!(rug_o, o);
        }

        let (sum_alt, o_alt) = add_prec_round_naive(
            x.clone(),
            y.clone(),
            max(x.significant_bits(), y.significant_bits()),
            rm,
        );
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    };
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Floor,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    // Note different behavior for Floor
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    // Note different behavior for Floor
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Down,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Up,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        RoundingMode::Exact,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        RoundingMode::Floor,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        RoundingMode::Ceiling,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        RoundingMode::Down,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        RoundingMode::Up,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        RoundingMode::Nearest,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        RoundingMode::Exact,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        RoundingMode::Floor,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        RoundingMode::Ceiling,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        RoundingMode::Down,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        RoundingMode::Up,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        RoundingMode::Nearest,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        RoundingMode::Exact,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Floor,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Ceiling,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Down,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Up,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Nearest,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Exact,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Floor,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Ceiling,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Down,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Up,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Nearest,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        RoundingMode::Exact,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );

    // - (round_bit != 0 || sticky_bit == 0) && (rm == Floor || rm == Down) in
    //   add_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    // - (round_bit != 0 || sticky_bit == 0) && (rm == Ceiling || rm == Up) in
    //   add_float_significands_same_prec_lt_w
    // - (rm == Ceiling || rm == Up) && sum == 0 in add_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Floor,
        "4.555806215962888",
        "0x4.8e4950f0795fc#53",
        Ordering::Less,
    );
    // - (rm == Ceiling || rm == Up) && sum != 0 in add_float_significands_same_prec_lt_w
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Ceiling,
        "4.555806215962889",
        "0x4.8e4950f079600#53",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Down,
        "4.555806215962888",
        "0x4.8e4950f0795fc#53",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Up,
        "4.555806215962889",
        "0x4.8e4950f079600#53",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Nearest,
        "4.555806215962888",
        "0x4.8e4950f0795fc#53",
        Ordering::Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Floor,
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Ceiling,
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Down,
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Up,
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Nearest,
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Exact,
        "-1.727379091216698",
        "-0x1.ba35842091e63#53",
        Ordering::Equal,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Floor,
        "1.727379091216698",
        "0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Ceiling,
        "1.727379091216698",
        "0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Down,
        "1.727379091216698",
        "0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Up,
        "1.727379091216698",
        "0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Nearest,
        "1.727379091216698",
        "0x1.ba35842091e63#53",
        Ordering::Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        RoundingMode::Exact,
        "1.727379091216698",
        "0x1.ba35842091e63#53",
        Ordering::Equal,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Floor,
        "-4.555806215962889",
        "-0x4.8e4950f079600#53",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Ceiling,
        "-4.555806215962888",
        "-0x4.8e4950f0795fc#53",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Down,
        "-4.555806215962888",
        "-0x4.8e4950f0795fc#53",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Up,
        "-4.555806215962889",
        "-0x4.8e4950f079600#53",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        RoundingMode::Floor,
        "-4.555806215962889",
        "-0x4.8e4950f079600#53",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        RoundingMode::Floor,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        RoundingMode::Ceiling,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        RoundingMode::Down,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        RoundingMode::Up,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );

    // Note different behavior for Floor
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    // - x_exp > y_exp in add_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    // - x_exp == y_exp in add_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        RoundingMode::Nearest,
        "2.0",
        "0x2.0#1",
        Ordering::Equal,
    );
    // - rm == Nearest && sum != 0 in add_float_significands_same_prec_lt_w
    test(
        "1.0",
        "0x1.0#3",
        "1.8",
        "0x1.c#3",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0#3",
        Ordering::Greater,
    );
    // - exp_diff >= Limb::WIDTH in add_float_significands_same_prec_lt_w
    test(
        "7.7e14",
        "0x2.bcE+12#8",
        "1.237e-9",
        "0x5.50E-8#8",
        RoundingMode::Nearest,
        "7.7e14",
        "0x2.bcE+12#8",
        Ordering::Less,
    );
    // - shift <= exp_diff < Limb::WIDTH in add_float_significands_same_prec_lt_w
    // - shift <= exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_lt_w
    test(
        "1.852193494e22",
        "0x3.ec137baE+18#29",
        "241425944.0",
        "0xe63de18.0#29",
        RoundingMode::Nearest,
        "1.852193494e22",
        "0x3.ec137baE+18#29",
        Ordering::Less,
    );
    test(
        "1.999999999999993",
        "0x1.fffffffffffe#48",
        "5.96046447753906e-8",
        "0x1.000000000000E-6#48",
        RoundingMode::Nearest,
        "2.00000005960464",
        "0x2.000001000000#48",
        Ordering::Greater,
    );

    // - in add_float_significands_same_prec_w
    // - x_exp == y_exp in add_float_significands_same_prec_w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0",
        "0x1.0000000000000000#64",
        RoundingMode::Nearest,
        "2.0",
        "0x2.0000000000000000#64",
        Ordering::Equal,
    );
    // - x_exp < y_exp in add_float_significands_same_prec_w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_w
    // - !overflow in add_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "2.0",
        "0x2.0000000000000000#64",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0000000000000000#64",
        Ordering::Equal,
    );
    // - x_exp > y_exp in add_float_significands_same_prec_w
    test(
        "2.0",
        "0x2.0000000000000000#64",
        "1.0",
        "0x1.0000000000000000#64",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0000000000000000#64",
        Ordering::Equal,
    );
    // - (round_bit != 0) || (sticky_bit != 0) && rm == Nearest in
    //   add_float_significands_same_prec_w
    // - round_bit == 0 || (sticky_bit == 0 && (sum & 1) == 0) in add_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        RoundingMode::Nearest,
        "2.0",
        "0x2.0000000000000000#64",
        Ordering::Less,
    );
    // - round_bit != 0 && (sticky_bit != 0 || (sum & 1) != 0) in add_float_significands_same_prec_w
    // - round_bit != 0 && (sticky_bit != 0 || (sum & 1) != 0) and !overflow in
    //   add_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000003",
        "0x1.0000000000000006#64",
        RoundingMode::Nearest,
        "2.0000000000000000004",
        "0x2.0000000000000008#64",
        Ordering::Greater,
    );
    // - exp_diff >= Limb::WIDTH in add_float_significands_same_prec_w
    test(
        "5.9376349676904431794e-6",
        "0x0.0000639df2b03f3e49a70#64",
        "2.9347251290514630352e-45",
        "0x1.0c11b075f03d6daeE-37#64",
        RoundingMode::Nearest,
        "5.9376349676904431794e-6",
        "0x0.0000639df2b03f3e49a70#64",
        Ordering::Less,
    );
    // - overflow in add_float_significands_same_prec_w
    test(
        "0.00022185253582909293959",
        "0x0.000e8a1162cbb1a4265#64",
        "0.000029745661521717034001",
        "0x0.0001f30ca4b8117ff0a0#64",
        RoundingMode::Nearest,
        "0.0002515981973508099736",
        "0x0.00107d1e0783c324170#64",
        Ordering::Greater,
    );
    // - round_bit != 0 && (sticky_bit != 0 || (sum & 1) != 0) and overflow in
    //   add_float_significands_same_prec_w
    test(
        "63.999999999999999997",
        "0x3f.ffffffffffffffc#64",
        "64.0",
        "0x40.000000000000000#64",
        RoundingMode::Nearest,
        "128.0",
        "0x80.00000000000000#64",
        Ordering::Greater,
    );

    // - (round_bit != 0) || (sticky_bit != 0) && (rm == Floor || rm == Down) in
    //   add_float_significands_same_prec_w
    // - (round_bit != 0) || (sticky_bit != 0) && (rm == Floor || rm == Down) && sum != 0 in
    //   add_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        RoundingMode::Down,
        "2.0",
        "0x2.0000000000000000#64",
        Ordering::Less,
    );
    // - (round_bit != 0) || (sticky_bit != 0) && (rm == Ceiling || rm == Up) in
    //   add_float_significands_same_prec_w
    // - (round_bit != 0) || (sticky_bit != 0) && (rm == Ceiling || rm == Up) && sum != 0 in
    //   add_float_significands_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        RoundingMode::Up,
        "2.0000000000000000002",
        "0x2.0000000000000004#64",
        Ordering::Greater,
    );
    // - (round_bit != 0) || (sticky_bit != 0) && (rm == Ceiling || rm == Up) && sum == 0 in
    //   add_float_significands_same_prec_w
    test(
        "536870911.99999999997",
        "0x1fffffff.ffffffffe#64",
        "1.00974195868289511071e-28",
        "0x7.ffffffffffffffe0E-24#64",
        RoundingMode::Up,
        "536870912.0",
        "0x20000000.000000000#64",
        Ordering::Greater,
    );

    // - in add_float_significands_same_prec_gt_w_lt_2w
    // - x_exp == y_exp in add_float_significands_same_prec_gt_w_lt_2w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.0",
        "0x1.0000000000000000#65",
        RoundingMode::Nearest,
        "2.0",
        "0x2.0000000000000000#65",
        Ordering::Equal,
    );
    // - x_exp < y_exp in add_float_significands_same_prec_gt_w_lt_2w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_gt_w_lt_2w
    // - exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "2.0",
        "0x2.0000000000000000#65",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0000000000000000#65",
        Ordering::Equal,
    );
    // - x_exp > y_exp in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "2.0",
        "0x2.0000000000000000#65",
        "1.0",
        "0x1.0000000000000000#65",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0000000000000000#65",
        Ordering::Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in add_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest in add_float_significands_same_prec_gt_w_lt_2w
    // - rm == Nearest && (sum_0 != 0 || sum_1 != 0) in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        RoundingMode::Nearest,
        "2.0",
        "0x2.0000000000000000#65",
        Ordering::Less,
    );
    // - rm == Nearest && sum_1 != 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000016",
        "0x1.0000000000000003#65",
        RoundingMode::Nearest,
        "2.0000000000000000002",
        "0x2.0000000000000004#65",
        Ordering::Greater,
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_w_lt_2w
    // - Limb::WIDTH < exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_w_lt_2w
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && !overflow in
    //   add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        "18.63123034252626794758647",
        "0x12.a1984fcd64a8ae228eef#85",
        RoundingMode::Nearest,
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        Ordering::Less,
    );
    // - exp_diff >= Limb::WIDTH * 2 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "4.8545822922649671226e27",
        "0xf.af9dc963a0709f78E+22#65",
        "1.14823551075108882469e-96",
        "0x2.73dea72af3fe6314E-80#65",
        RoundingMode::Nearest,
        "4.8545822922649671226e27",
        "0xf.af9dc963a0709f78E+22#65",
        Ordering::Less,
    );
    // - exp_diff == Limb::WIDTH in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "19585.2851423168986928116147584507795",
        "0x4c81.48ff163dc91a0d4bd90309b0f8#116",
        "372369974082165972902790.766638151683",
        "0x4eda377c7f0d747fa386.c44265dd58#116",
        RoundingMode::Nearest,
        "372369974082165972922376.05178046858",
        "0x4eda377c7f0d747ff008.0d417c1b20#116",
        Ordering::Less,
    );
    // - exp_diff < Limb::WIDTH && overflow in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "18.492649216180435830000529",
        "0x12.7e1e424fe51f1bb914c0#85",
        "56.637589789906471403844847",
        "0x38.a339159fe96c1722fdfe#85",
        RoundingMode::Nearest,
        "75.130239006086907233845378",
        "0x4b.215757efce8b32dc12c0#85",
        Ordering::Greater,
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && overflow in
    //   add_float_significands_same_prec_gt_w_lt_2w
    test(
        "5.29395592276605355108231857701752e-23",
        "0x4.00000007e000fffffff0000000E-19#107",
        "255.999999999999999999999947060441",
        "0xff.ffffffffffffffffffc000000#107",
        RoundingMode::Nearest,
        "256.0",
        "0x100.0000000000000000000000000#107",
        Ordering::Less,
    );
    // - rm == Nearest && sum_1 == 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "0.0000610351562499999999996",
        "0x0.0003ffffffffffffffff8#67",
        "17179869183.9999389648",
        "0x3ffffffff.fffc00000#67",
        RoundingMode::Nearest,
        "17179869184.0",
        "0x400000000.00000000#67",
        Ordering::Greater,
    );

    // - rm == Floor || rm == Down in add_float_significands_same_prec_gt_w_lt_2w
    // - (rm == Floor || rm == Down) && (sum_0 != 0 || sum_1 != 0) in
    //   add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        RoundingMode::Floor,
        "2.0",
        "0x2.0000000000000000#65",
        Ordering::Less,
    );
    // - rm == Ceiling || rm == Up in add_float_significands_same_prec_gt_w_lt_2w
    // - rm == Ceiling || rm == Up && sum_1 != 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        RoundingMode::Ceiling,
        "2.0000000000000000001",
        "0x2.0000000000000002#65",
        Ordering::Greater,
    );
    // - rm == Ceiling || rm == Up && sum_1 == 0 in add_float_significands_same_prec_gt_w_lt_2w
    test(
        "1.9999999999999999999999998",
        "0x1.ffffffffffffffffffffc#83",
        "2.4074118565121938372272894e-35",
        "0x1.fffff8000000007fffff8E-29#83",
        RoundingMode::Ceiling,
        "2.0",
        "0x2.000000000000000000000#83",
        Ordering::Greater,
    );

    // - in add_float_significands_same_prec_2w
    // - x_exp == y_exp in add_float_significands_same_prec_2w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        RoundingMode::Nearest,
        "2.0",
        "0x2.00000000000000000000000000000000#128",
        Ordering::Equal,
    );
    // - x_exp < y_exp in add_float_significands_same_prec_2w
    // - exp_diff < TWICE_WIDTH in add_float_significands_same_prec_2w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_2w
    // - exp_diff < TWICE_WIDTH && !overflow in add_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "2.0",
        "0x2.00000000000000000000000000000000#128",
        RoundingMode::Nearest,
        "3.0",
        "0x3.00000000000000000000000000000000#128",
        Ordering::Equal,
    );
    // - x_exp > y_exp in add_float_significands_same_prec_2w
    test(
        "2.0",
        "0x2.00000000000000000000000000000000#128",
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        RoundingMode::Nearest,
        "3.0",
        "0x3.00000000000000000000000000000000#128",
        Ordering::Equal,
    );
    // - round_bit != 0 || sticky_bit != 0 in add_float_significands_same_prec_2w
    // - rm == Nearest in add_float_significands_same_prec_2w
    // - rm == Nearest && (round_bit == 0 || (sticky_bit == 0 && (sum_0 & 1) == 0)) in
    //   add_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        RoundingMode::Nearest,
        "2.0",
        "0x2.00000000000000000000000000000000#128",
        Ordering::Less,
    );
    // - rm == Nearest && round_bit != 0 && (sticky_bit != 0 || (sum_0 & 1) != 0) in
    //   add_float_significands_same_prec_2w
    // - rm == Nearest && !overflow in add_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000018",
        "0x1.00000000000000000000000000000006#128",
        RoundingMode::Nearest,
        "2.00000000000000000000000000000000000002",
        "0x2.00000000000000000000000000000008#128",
        Ordering::Greater,
    );
    // - exp_diff >= TWICE_WIDTH in add_float_significands_same_prec_2w
    // - exp_diff > TWICE_WIDTH in add_float_significands_same_prec_2w
    test(
        "5.80991149045382428948889299639419733262e-6",
        "0x0.00006179613d776a1c835894818a219f488e8#128",
        "5.07801249136957145270807726205511855421e-45",
        "0x1.cfd8608b7c32de2bbcfecf8bcf8a2d00E-37#128",
        RoundingMode::Nearest,
        "5.80991149045382428948889299639419733262e-6",
        "0x0.00006179613d776a1c835894818a219f488e8#128",
        Ordering::Less,
    );
    // - Limb::WIDTH <= exp_diff < TWICE_WIDTH in add_float_significands_same_prec_2w
    // - Limb::WIDTH < exp_diff < TWICE_WIDTH in add_float_significands_same_prec_2w
    test(
        "4354249796990942.35435357526597783143164",
        "0xf782ac869b7de.5ab6ea78fcf0cc5079f#128",
        "8.03239453825726512240307053405256016022e-10",
        "0x3.732bce7aa121827a284545a25f32dc68E-8#128",
        RoundingMode::Nearest,
        "4354249796990942.35435357606921728525736",
        "0xf782ac869b7de.5ab6ea7c701c9acb1b1#128",
        Ordering::Less,
    );
    // - exp_diff == TWICE_WIDTH in add_float_significands_same_prec_2w
    test(
        "15732412727332569995335732833027757624.44",
        "0xbd5f3d586bc01069a1d94f5ab5a1638.7#128",
        "0.0373708302820085888760745841639896128921",
        "0x0.0991227de2b63edc67164401ce8ebdb04#128",
        RoundingMode::Nearest,
        "15732412727332569995335732833027757624.5",
        "0xbd5f3d586bc01069a1d94f5ab5a1638.8#128",
        Ordering::Greater,
    );
    // - Limb::WIDTH == exp_diff in add_float_significands_same_prec_2w
    test(
        "1.057437459917463716438672572710788562701e-17",
        "0xc.310127aae1df1a1cb12f60c4d339d76E-15#128",
        "148.0549133677002965445211858794413066474",
        "0x94.0e0ecd6e62d0a8c7c7c2a633277e3e#128",
        RoundingMode::Nearest,
        "148.054913367700296555095560478615943812",
        "0x94.0e0ecd6e62d0a98ad7d520e1456fe0#128",
        Ordering::Greater,
    );
    // - exp_diff < TWICE_WIDTH && overflow in add_float_significands_same_prec_2w
    test(
        "990.890284854484258981204316304960898664",
        "0x3de.e3e9b54e224e900a8701c94cea27bc#128",
        "111.972242543885876168914754084523121772",
        "0x6f.f8e4e329c509f04b7f9497ec8ce6438#128",
        RoundingMode::Nearest,
        "1102.862527398370135150119070389484020437",
        "0x44e.dcce9877e758805606966139770e00#128",
        Ordering::Greater,
    );
    // - rm == Nearest && overflow in add_float_significands_same_prec_2w
    test(
        "1152920954851033088.0",
        "0xfffff8000000000.00000000000000000#128",
        "549755813887.999999999999999999999999998",
        "0x7fffffffff.ffffffffffffffffffffff8#128",
        RoundingMode::Nearest,
        "1152921504606846976.0",
        "0x1000000000000000.00000000000000000#128",
        Ordering::Greater,
    );
    // - rm == Floor || m == Down in add_float_significands_same_prec_2w
    // - (rm == Floor || m == Down) && (sum_0 != 0 || sum_1 != 0) in
    //   add_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        RoundingMode::Floor,
        "2.0",
        "0x2.00000000000000000000000000000000#128",
        Ordering::Less,
    );
    // - rm == Ceiling || m == Up in add_float_significands_same_prec_2w
    // - (rm == Ceiling || m == Up) && !overflow in add_float_significands_same_prec_2w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#128",
        "1.000000000000000000000000000000000000006",
        "0x1.00000000000000000000000000000002#128",
        RoundingMode::Ceiling,
        "2.00000000000000000000000000000000000001",
        "0x2.00000000000000000000000000000004#128",
        Ordering::Greater,
    );
    // - (rm == Ceiling || m == Up) && overflow in add_float_significands_same_prec_2w
    test(
        "69631.9999999850988390281969486491823076",
        "0x10fff.ffffffc000000ffffffff80007fc#128",
        "1.255420347077336152767157884641533283217e58",
        "0x1.fffffffffffffffffffffffffffffffeE+48#128",
        RoundingMode::Ceiling,
        "1.25542034707733615276715788464153328322e58",
        "0x2.00000000000000000000000000000000E+48#128",
        Ordering::Greater,
    );

    // - (round_bit != 0 || sticky_bit != 0) && (rm == Ceiling || rm == Up) in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && (rm == Ceiling || rm == Up) && sum_2 != 0 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000009",
        "0x1.00000000000000000000000000000003#129",
        RoundingMode::Up,
        "2.000000000000000000000000000000000000012",
        "0x2.00000000000000000000000000000004#129",
        Ordering::Greater,
    );
    test(
        "9903517953099800764370386944.0000000000291038304566",
        "0x1fffff800000000000000000.000000001fffffffff8#166",
        "1.68499666669691498716668845382709804457445989930032e66",
        "0x1.0000000000000000000001fff80000000000000000E+55#166",
        RoundingMode::Up,
        "1.68499666669691498716668845382709804458436341725346e66",
        "0x1.0000000000000000000001fff8000001fffff80008E+55#166",
        Ordering::Greater,
    );
    // - in add_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH in add_float_significands_same_prec_gt_2w_lt_3w
    // - exp_diff < Limb::WIDTH && !overflow in add_float_significands_same_prec_gt_2w_lt_3w
    // - round_bit != 0 || sticky_bit != 0 in add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && (round_bit == 0 || (sticky_bit == 0
    //   && (sum_0 & shift_bit) == 0)) in add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && (round_bit == 0 || (sticky_bit == 0
    //   && (sum_0 & shift_bit) == 0)) && sum != 0 in add_float_significands_same_prec_gt_2w_lt_3w
    // - x_exp > y_exp in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.0",
        "0x2.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        RoundingMode::Nearest,
        "3.0",
        "0x3.00000000000000000000000000000000#129",
        Ordering::Less,
    );
    // - x_exp < y_exp in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        "2.0",
        "0x2.00000000000000000000000000000000#129",
        RoundingMode::Nearest,
        "3.0",
        "0x3.00000000000000000000000000000000#129",
        Ordering::Less,
    );
    // - x_exp == y_exp in add_float_significands_same_prec_gt_2w_lt_3w
    // - round_bit == 0 && sticky_bit == 0 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        RoundingMode::Nearest,
        "2.0",
        "0x2.00000000000000000000000000000000#129",
        Ordering::Equal,
    );
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && round_bit != 0 && (sticky_bit != 0
    //   || (sum_0 & shift_bit) != 0) in add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && round_bit != 0 && (sticky_bit != 0
    //   || (sum_0 & shift_bit) != 0) && sum != 0 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000009",
        "0x1.00000000000000000000000000000003#129",
        RoundingMode::Nearest,
        "2.000000000000000000000000000000000000012",
        "0x2.00000000000000000000000000000004#129",
        Ordering::Greater,
    );
    // - Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH * 2 < exp_diff < Limb::WIDTH * 3 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3 && !overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "2.024076700393272432111968987625898501371897741e-29",
        "0x1.9a88122864b9c4b577e4b655958954f82345dE-24#149",
        "245906107849378561117126906.9059035528266331265",
        "0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
        RoundingMode::Nearest,
        "245906107849378561117126906.9059035528266331265",
        "0xcb68a4d1611415054400fa.e7e94b94b8791630#149",
        Ordering::Less,
    );
    // - exp_diff >= Limb::WIDTH * 3 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.397610888919711045634814958598336677777534377e47",
        "0x4.d0791b9428a6b4fc52e44e537ab5a0f269ad60E+39#155",
        "6.8892360159362421595728818935378487832685754059e-50",
        "0x1.9c693c182df3035eef00d41638bbdd942f4d498E-41#155",
        RoundingMode::Nearest,
        "4.397610888919711045634814958598336677777534377e47",
        "0x4.d0791b9428a6b4fc52e44e537ab5a0f269ad60E+39#155",
        Ordering::Less,
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH < exp_diff < Limb::WIDTH * 2 in add_float_significands_same_prec_gt_2w_lt_3w
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && !overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4.2850537238606374652351877988811796373898773e-22",
        "0x2.0607fd4819748c532aad3528693c1e3c1966E-18#146",
        "978.49328809934495391839880801989439981236569",
        "0x3d2.7e4820fe314caadcb9a156bef2f1c8e53c#146",
        RoundingMode::Nearest,
        "978.49328809934495391839923652526678587611222",
        "0x3d2.7e4820fe314caadcbba75ebc3b0b3d718f#146",
        Ordering::Less,
    );
    // - exp_diff < Limb::WIDTH && overflow in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "4499918.46219262840948309077470961400786337",
        "0x44a9ce.7652418f789422bc22220831e2030#137",
        "64560208.0262619516023687759351781439347886",
        "0x3d91c50.06b91a6f42e5205070f82f89eefa#137",
        RoundingMode::Nearest,
        "69060126.488454580011851866709887757942652",
        "0x41dc61e.7d0b5bfebb79430c931a37bbd0fc#137",
        Ordering::Less,
    );
    // - exp_diff == Limb::WIDTH * 2 in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "563971925627753843356041629019151473018178607215.42",
        "0x62c960337e963a378ba6626ea422d8a5e623986f.6c#165",
        "1301375421.83361702516620516356439489325145225661938",
        "0x4d9169bd.d567ece47a47ef60371d48c969ba8765d4#165",
        RoundingMode::Nearest,
        "563971925627753843356041629019151473019479982637.25",
        "0x62c960337e963a378ba6626ea422d8a633b5022d.40#165",
        Ordering::Less,
    );
    // - exp_diff == Limb::WIDTH in add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "226.9305090753243797994707628568605406194",
        "0xe2.ee35d7bf263fda8c632644ad7c49d98#130",
        "4262448175090788889452.984188256984861391",
        "0xe71159efd3a67e736c.fbf3c2f8db72fb8#130",
        RoundingMode::Nearest,
        "4262448175090788889679.91469733230924119",
        "0xe71159efd3a67e744f.ea299ab801b2d60#130",
        Ordering::Less,
    );
    // - Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 && overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1180591620717411303423.999999999999999999999999997",
        "0x3fffffffffffffffff.ffffffffffffffffffffff#158",
        "5.68434188616351954822429632036496545806324230121e-14",
        "0x1.000000000fffffffffffe0000000000000000038E-11#158",
        RoundingMode::Nearest,
        "1180591620717411303424.00000000000005684341886163",
        "0x400000000000000000.00000000001000000000fe#158",
        Ordering::Less,
    );
    // - Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3 && overflow in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && rm == Nearest && round_bit != 0 && (sticky_bit != 0
    //   || (sum_0 & shift_bit) != 0) && sum == 0
    test(
        "4503599627370495.9999999999999999999999999996",
        "0xfffffffffffff.ffffffffffffffffffffffe#143",
        "3.3087224509824797385046520537834728287650668e-24",
        "0x4.00000003ffff0000000000000000fffffffE-20#143",
        RoundingMode::Nearest,
        "4503599627370496.000000000000000000000003309",
        "0x10000000000000.00000000000000000004000#143",
        Ordering::Greater,
    );
    // - (round_bit != 0 || sticky_bit != 0) && (rm == Floor || rm == Down) in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    // - (round_bit != 0 || sticky_bit != 0) && (rm == Floor || rm == Down) && sum != 0 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        "1.000000000000000000000000000000000000003",
        "0x1.00000000000000000000000000000001#129",
        RoundingMode::Down,
        "2.0",
        "0x2.00000000000000000000000000000000#129",
        Ordering::Less,
    );
    // - (round_bit != 0 || sticky_bit != 0) && (rm == Ceiling || rm == Up) && sum_2 == 0 in
    //   add_float_significands_same_prec_gt_2w_lt_3w
    test(
        "18014398509481983.9999999999999999999999999999999998",
        "0x3fffffffffffff.ffffffffffffffffffffffffffff#166",
        "9.6296497219361792652798897129246365926905082410768e-35",
        "0x7.ffffffffffffffffffffffffffffffffffffffffeE-29#166",
        RoundingMode::Up,
        "18014398509481984.0",
        "0x40000000000000.0000000000000000000000000000#166",
        Ordering::Greater,
    );

    // - 0 < exp_diff < prec && (rm == Ceiling || rm == Up) in
    //   add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && (rm == Ceiling || rm == Up) && round_bit == 0 && sticky_bit == 0 in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Up,
        "3.0",
        "0x3.000000000000000000000000000000000000000000000000#192",
        Ordering::Equal,
    );
    // - in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && round_bit == 0 in add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Nearest,
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        Ordering::Equal,
    );
    // - x_exp < y_exp in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec in add_float_significands_same_prec_ge_3w
    // - in add_significands_rsh_to_out
    // - exp_diff < Limb::WIDTH in add_significands_rsh_to_out
    // - 0 < exp_diff < prec && shift == 0 in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && limb == 0 in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && rm == Nearest in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && rm == Nearest && round_bit == 0 in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Nearest,
        "3.0",
        "0x3.000000000000000000000000000000000000000000000000#192",
        Ordering::Equal,
    );
    // - x_exp > y_exp in add_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Nearest,
        "3.0",
        "0x3.000000000000000000000000000000000000000000000000#192",
        Ordering::Equal,
    );
    // - 0 < exp_diff < prec && shift != 0 in add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#193",
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#193",
        RoundingMode::Nearest,
        "3.0",
        "0x3.000000000000000000000000000000000000000000000000#193",
        Ordering::Equal,
    );
    // - exp_diff == 0 && rm == Nearest in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && rm == Nearest && out[0] & shift_bit == 0 in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        RoundingMode::Nearest,
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        Ordering::Less,
    );
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && sticky_bit == 0 && out[0] &
    //   shift_bit == 0 in add_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        RoundingMode::Nearest,
        "3.0",
        "0x3.000000000000000000000000000000000000000000000000#192",
        Ordering::Less,
    );
    // - exp_diff == 0 && rm == Nearest && out[0] & shift_bit != 0 in
    //   add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && rm == Nearest && out[0] & shift_bit != 0 && !carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.000000000000000000000000000000000000000000000000000000001",
        "0x1.000000000000000000000000000000000000000000000006#192",
        RoundingMode::Nearest,
        "2.0000000000000000000000000000000000000000000000000000000013",
        "0x2.000000000000000000000000000000000000000000000008#192",
        Ordering::Greater,
    );
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] &
    //   shift_bit != 0) in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] &
    //   shift_bit != 0) && !carry in add_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.000000000000000000000000000000000000000000000000000000001",
        "0x1.000000000000000000000000000000000000000000000006#192",
        RoundingMode::Nearest,
        "3.0000000000000000000000000000000000000000000000000000000013",
        "0x3.000000000000000000000000000000000000000000000008#192",
        Ordering::Greater,
    );
    // - exp_diff >= Limb::WIDTH in add_significands_rsh_to_out
    // - exp_diff >= Limb::WIDTH && r != 0 in add_significands_rsh_to_out
    test(
        "7.28057116938384227432903448367767196428679514765398378973101e-48",
        "0xa.a3fc2da1f20fb2d9771f86d3c16a444cd62d5d139e3935f24E-40#198",
        "3.5123473778825578958968695187657587760357139395948269588971e-27",
        "0x1.1646de419a6dbd3466f3081403a87d719b7a765a1ec69e4658E-22#198",
        RoundingMode::Nearest,
        "3.5123473778825578959041500899351426183100429740785046308614e-27",
        "0x1.1646de419a6dbd3471970441a59a8d2474f195e0f288088aa8E-22#198",
        Ordering::Greater,
    );
    // - exp_diff >= prec in add_float_significands_same_prec_ge_3w
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w && (rm == Nearest || rm == Floor
    //   || rm == Down)
    test(
        "4.1322282880219162156901559575161649173615955518072607291207e86",
        "0xd.4b575f05941ee41ef3ef9a37068d9d453f22eb3bf80bd1b0E+71#193",
        "0.023991386767031193042066748710708351501952890752924613005724",
        "0x0.06244cad8cd272134e34b325815ad281733f2c06231a0ee744#193",
        RoundingMode::Nearest,
        "4.1322282880219162156901559575161649173615955518072607291207e86",
        "0xd.4b575f05941ee41ef3ef9a37068d9d453f22eb3bf80bd1b0E+71#193",
        Ordering::Less,
    );
    // - 0 < exp_diff < prec && limb != 0 in add_float_significands_same_prec_ge_3w
    test(
        "8.699772042374378140693728074838279708562673799416097107796",
        "0x8.b32442b4a730454d66b1b2bdf7a2863d417e6ff22d7f6c58#193",
        "7.5897463681962395437740598844462353563682906392115908083148",
        "0x7.96f99e34566e7be1960d023e431dc5e0a7ad24ad691a1ac4#193",
        RoundingMode::Nearest,
        "16.289518410570617684467787959284515064930964438627687916112",
        "0x10.4a1de0e8fd9ec12efcbeb4fc3ac04c1de92b949f9699872#193",
        Ordering::Greater,
    );
    // - exp_diff >= Limb::WIDTH && r == 0 in add_significands_rsh_to_out
    test(
        "6.442552350746554109885349691592991892989624685631192235549e-6",
        "0x0.00006c168d38e231899f0fc85d1888549d5177bdceaee72e15060#192",
        "1476808010161862576835936576709144.7975622615653024045505082",
        "0x48cff00a780a50d34bb694ada218.cc2d0a55f25f9f9126258#192",
        RoundingMode::Nearest,
        "1476808010161862576835936576709144.797568704117653151104618",
        "0x48cff00a780a50d34bb694ada218.cc2d766c7f9881c2afc48#192",
        Ordering::Less,
    );
    // - exp_diff == prec in add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest in add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest && power && !carry in
    //   add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest && !power in add_float_significands_same_prec_ge_3w
    test(
        "4.0635838402455207229400698207668893925379768151364313942222e-23",
        "0x3.1202ecf10ff40b477337957dede18bd7b746884ec977474eE-19#194",
        "1174582238252884689829665592721065057.76655867827770290150723",
        "0xe237601fa3ed6d89b0ae33e924c461.c43d3085aaefab6b5d4#194",
        RoundingMode::Nearest,
        "1174582238252884689829665592721065057.76655867827770290150729",
        "0xe237601fa3ed6d89b0ae33e924c461.c43d3085aaefab6b5d8#194",
        Ordering::Greater,
    );
    // - 0 < exp_diff < prec && rm == Nearest && round_bit != 0 && (sticky_bit != 0 || out[0] &
    //   shift_bit != 0) && carry in add_float_significands_same_prec_ge_3w
    test(
        "4.336808689942017736029811203479766845699938816177735095446e-19",
        "0x7.fffffffffffffffffffffffffffffffe0000000000000000E-16#192",
        "5192296858534827628530496329220095.999999999999999999566319",
        "0xffffffffffffffffffffffffffff.fffffffffffffff80000#192",
        RoundingMode::Nearest,
        "5192296858534827628530496329220096.0",
        "0x10000000000000000000000000000.00000000000000000000#192",
        Ordering::Greater,
    );
    // - exp_diff == prec && rm == Nearest && power && carry in
    //   add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && rm == Nearest && !power && carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "158456325028528675187087900671.99999999999999999999999999997",
        "0x1ffffffffffffffffffffffff.fffffffffffffffffffffffe#192",
        "2.5243548967072377773175314089049159349542605923488736152645e-29",
        "0x1.fffffffffffffffffffffffffffffffffffffffffffffffeE-24#192",
        RoundingMode::Nearest,
        "158456325028528675187087900672.0",
        "0x2000000000000000000000000.000000000000000000000000#192",
        Ordering::Greater,
    );
    // - exp_diff == prec && rm == Nearest && power in add_float_significands_same_prec_ge_3w
    test(
        "332306998926888516295359133097394175.99999997019767761230469",
        "0x3ffffffff0007fffffffffffffffff.ffffff8000000000000#192",
        "2.6469779601696885595885078146238811314105987548828125e-23",
        "0x2.000000000000000000000000000000000000000000000000E-19#192",
        RoundingMode::Nearest,
        "332306998926888516295359133097394175.99999997019767761230469",
        "0x3ffffffff0007fffffffffffffffff.ffffff8000000000000#192",
        Ordering::Less,
    );
    // - 0 < exp_diff < prec && (rm == Floor || rm == Down || rm == Exact) in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        RoundingMode::Down,
        "3.0",
        "0x3.000000000000000000000000000000000000000000000000#192",
        Ordering::Equal,
    );
    // - exp_diff == 0 && (rm == Floor || rm == Down) in add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        RoundingMode::Down,
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        Ordering::Less,
    );
    // - exp_diff == 0 && (rm == Ceiling || rm == Up) in add_float_significands_same_prec_ge_3w
    // - exp_diff == 0 && (rm == Ceiling || rm == Up) && !carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.0",
        "0x1.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        RoundingMode::Up,
        "2.0000000000000000000000000000000000000000000000000000000006",
        "0x2.000000000000000000000000000000000000000000000004#192",
        Ordering::Greater,
    );
    // - 0 < exp_diff < prec && (rm == Ceiling || rm == Up) && (round_bit != 0 || sticky_bit != 0)
    //   in add_float_significands_same_prec_ge_3w
    // - 0 < exp_diff < prec && (rm == Ceiling || rm == Up) && (round_bit != 0 || sticky_bit != 0)
    //   && !carry in add_float_significands_same_prec_ge_3w
    test(
        "2.0",
        "0x2.000000000000000000000000000000000000000000000000#192",
        "1.0000000000000000000000000000000000000000000000000000000003",
        "0x1.000000000000000000000000000000000000000000000002#192",
        RoundingMode::Up,
        "3.0000000000000000000000000000000000000000000000000000000006",
        "0x3.000000000000000000000000000000000000000000000004#192",
        Ordering::Greater,
    );
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w && (rm == Ceiling || rm == Up)
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w && (rm == Ceiling || rm == Up) &&
    //   !carry
    test(
        "6823.472967851766873629348006893003460678376513514025568927",
        "0x1aa7.79146bcf65e9c10b73dc31b712bdbba94db27f42827ee#192",
        "3.4171243258195824440860481554099490634319461554553884152664e-68",
        "0xe.bd75c60b3fb1b2daadd125b174611af23cd95ed37b18fd3E-57#192",
        RoundingMode::Up,
        "6823.472967851766873629348006893003460678376513514025568928",
        "0x1aa7.79146bcf65e9c10b73dc31b712bdbba94db27f42827f0#192",
        Ordering::Greater,
    );
    // - exp_diff == prec && (rm == Ceiling || rm == Up) in add_float_significands_same_prec_ge_3w
    // - exp_diff == prec && (rm == Ceiling || rm == Up) && !carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "1.1549982013361157285883413763473468330143594437077681839568e-64",
        "0xc.29de762f7d1efb4e2c76c8e4086645c726cd7efd160d9b2E-54#192",
        "7.183764682683218761534928278745259569270911336851315880289e-7",
        "0xc.0d6747ace11077b45ef60fe0663937937659be6facba820E-6#192",
        RoundingMode::Up,
        "7.183764682683218761534928278745259569270911336851315880291e-7",
        "0xc.0d6747ace11077b45ef60fe0663937937659be6facba821E-6#192",
        Ordering::Greater,
    );
    // - exp_diff == prec && (rm == Floor || rm == Down) in add_float_significands_same_prec_ge_3w
    test(
        "1.633185017652497317802829911277029120405335932425346213043e-62",
        "0x6.b7f1cf4acb21f3fca0c966202fee44bb9bb293511aa1d780E-52#192",
        "0.00010303969992625256008619861293450462215178250710705317873879",
        "0x0.0006c0b8243103d1ef7ab2f1e9a66ec9ee623a5e72e237199db8#192",
        RoundingMode::Floor,
        "0.00010303969992625256008619861293450462215178250710705317873879",
        "0x0.0006c0b8243103d1ef7ab2f1e9a66ec9ee623a5e72e237199db8#192",
        Ordering::Less,
    );
    // - exp_diff > prec in add_float_significands_same_prec_ge_3w && (rm == Ceiling || rm == Up) &&
    //   carry
    test(
        "2.3509707655716138708899159999241985031053943929132983433321e-38",
        "0x7.fffc0007ffffffff0000000007ffffffffffffffffffffffE-32#195",
        "2.15679573337205118357336120696157045389097155380324579848825e68",
        "0x7.ffffffffffffffffffffffffffffffffffffffffffffffffE+56#195",
        RoundingMode::Up,
        "2.1567957333720511835733612069615704538909715538032457984883e68",
        "0x8.000000000000000000000000000000000000000000000000E+56#195",
        Ordering::Greater,
    );
    // - 0 < exp_diff < prec && (rm == Ceiling || rm == Up) && (round_bit != 0 || sticky_bit != 0)
    //   && carry in add_float_significands_same_prec_ge_3w
    test(
        "8388607.9999999995343387126922607421875",
        "0x7fffff.fffffffe00000000000000000000000000000000000#192",
        "4.6566128730773925781249999999999999999999999999999999999993e-10",
        "0x1.fffffffffffffffffffffffffffffffffffffffffffffffeE-8#192",
        RoundingMode::Up,
        "8388608.0",
        "0x800000.000000000000000000000000000000000000000000#192",
        Ordering::Greater,
    );
    // - exp_diff == prec && (rm == Ceiling || rm == Up) && carry in
    //   add_float_significands_same_prec_ge_3w
    test(
        "511.9999999999999999999999999999999999999999999999999999999",
        "0x1ff.fffffffffffffffffffffffffffffffffffffffffffffe#192",
        "8.156630584998155601789981346010670828251902552640272418926e-56",
        "0x1.ffffffffffffff00001ffffffffffffff80003f003000000E-46#192",
        RoundingMode::Up,
        "512.0",
        "0x200.0000000000000000000000000000000000000000000000#192",
        Ordering::Greater,
    );

    // - rm == Floor || rm == Down in add_float_significands_same_prec_general_round
    test(
        "1.0",
        "0x1.0#1",
        "1.5",
        "0x1.8#2",
        RoundingMode::Down,
        "2.0",
        "0x2.0#2",
        Ordering::Less,
    );
    // - rm == Ceiling || rm == Up in add_float_significands_same_prec_general_round
    // - (rm == Ceiling || rm == Up) && !carry in add_float_significands_same_prec_general_round
    test(
        "1.0",
        "0x1.0#1",
        "1.5",
        "0x1.8#2",
        RoundingMode::Up,
        "3.0",
        "0x3.0#2",
        Ordering::Greater,
    );
    // - (rm == Ceiling || rm == Up) && carry in add_float_significands_same_prec_general_round
    test(
        "2.0",
        "0x2.0#1",
        "1.5",
        "0x1.8#2",
        RoundingMode::Up,
        "4.0",
        "0x4.0#2",
        Ordering::Greater,
    );

    // - shift2 == 0 in add_float_significands_same_prec_general
    // - y in add_float_significands_same_prec_general
    // - shift != 0 in add_float_significands_same_prec_general
    // - x == 0 first time in add_float_significands_same_prec_general
    // - shift == 0 || following_bits != Uninitialized in add_float_significands_same_prec_general
    // - round_bit != Uninitialized || shift == 0 in add_float_significands_same_prec_general
    // - exp_diff_rem == 0 && yi == 0 second time in add_float_significands_same_prec_general
    // - round_bit != Uninitialized sixth time in add_float_significands_same_prec_general
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#2",
        RoundingMode::Nearest,
        "2.0",
        "0x2.0#2",
        Ordering::Equal,
    );
    // - following_bits != False || round_bit != False in
    //   add_float_significands_same_prec_general_round
    // - rm == Nearest in add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits == False in add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits == False && out[0] & shift_bit == 0 in
    //   add_float_significands_same_prec_general_round
    test(
        "1.0",
        "0x1.0#1",
        "1.5",
        "0x1.8#2",
        RoundingMode::Nearest,
        "2.0",
        "0x2.0#2",
        Ordering::Less,
    );
    // - rm == Nearest && following_bits == False && out[0] & shift_bit != 0 in
    //   add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits == False && out[0] & shift_bit != 0 && carry in
    //   add_float_significands_same_prec_general_round
    test(
        "2.0",
        "0x2.0#1",
        "1.5",
        "0x1.8#2",
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#2",
        Ordering::Greater,
    );
    // - rm == Nearest && following_bits == False && out[0] & shift_bit != 0 && !carry in
    //   add_float_significands_same_prec_general_round
    test(
        "1.0",
        "0x1.0#1",
        "1.8",
        "0x1.c#3",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0#3",
        Ordering::Greater,
    );
    // - x != 0 && x != mask second time in add_float_significands_same_prec_general
    // - rm == Nearest && following_bits != False && round_bit != False in
    //   add_float_significands_same_prec_general_round
    // - rm == Nearest && following_bits != False && round_bit != False && !carry in
    //   add_float_significands_same_prec_general_round
    test(
        "1.5",
        "0x1.8#2",
        "4.0",
        "0x4.0#1",
        RoundingMode::Nearest,
        "6.0",
        "0x6.0#2",
        Ordering::Greater,
    );
    // - rm == Nearest && following_bits != False && round_bit == False in
    //   add_float_significands_same_prec_general_round
    test(
        "4.0",
        "0x4.0#1",
        "1.2",
        "0x1.4#3",
        RoundingMode::Nearest,
        "5.0",
        "0x5.0#3",
        Ordering::Less,
    );
    // - x != 0 && x != mask first time in add_float_significands_same_prec_general
    // - shift != 0 && following_bits == Uninitialized in add_float_significands_same_prec_general
    test(
        "1.2",
        "0x1.4#3",
        "3.0",
        "0x3.0#2",
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#3",
        Ordering::Less,
    );
    // - rm == Nearest && following_bits != False && round_bit != False && carry in
    //   add_float_significands_same_prec_general_round
    test(
        "1.8",
        "0x1.c#3",
        "6.0",
        "0x6.0#2",
        RoundingMode::Nearest,
        "8.0",
        "0x8.0#3",
        Ordering::Greater,
    );
    // - out_bits <= exp_diff in add_float_significands_same_prec_general
    // - out_len <= xs_len first time in add_float_significands_same_prec_general
    // - difw > 0 && difw > ys_len && exp_diff > out_bits in
    //   add_float_significands_same_prec_general
    // - round_bit != Uninitialized fifth time in add_float_significands_same_prec_general
    test(
        "8.82188e11",
        "0xc.d668E+9#18",
        "9.75459983374e122",
        "0x1.79c17f063aE+102#40",
        RoundingMode::Nearest,
        "9.75459983374e122",
        "0x1.79c17f063aE+102#40",
        Ordering::Less,
    );
    // - out_len > xs_len first time in add_float_significands_same_prec_general
    test(
        "2.8577648979177105962332201291018926848163080599637e-19",
        "0x5.458a93bffa7b1c05bdd1c0552b60196746d9083cE-16#162",
        "3.569720699507868e50",
        "0xf.4400d3acf388E+41#51",
        RoundingMode::Nearest,
        "3.5697206995078675404584127554321345196383736430592e50",
        "0xf.4400d3acf3880000000000000000000000000000E+41#162",
        Ordering::Less,
    );
    // - overlap > ys_len in add_float_significands_same_prec_general
    // - out_len - k > overlap in add_float_significands_same_prec_general
    // - difw <= 0 || difw <= ys_len in add_float_significands_same_prec_general
    // - round_bit != Uninitialized fourth time in add_float_significands_same_prec_general
    test(
        "29780282551.762684458936866363165",
        "0x6ef0b0cb7.c33f49e84d21bb6040#104",
        "0.00003945598947538",
        "0x0.000295f62f36adb#46",
        RoundingMode::Nearest,
        "29780282551.762723914926341743141",
        "0x6ef0b0cb7.c341dfde7c58691040#104",
        Ordering::Equal,
    );
    // - out_len > xs_len second time in add_float_significands_same_prec_general
    test(
        "1.07183972513958531257713938927815e-11",
        "0xb.c8f5eafa12eb9821601f1dd6aeE-10#107",
        "0.00374222828352849",
        "0x0.00f5402c178824#46",
        RoundingMode::Nearest,
        "0.00374222829424688982311482391965285",
        "0x0.00f5402c235119eafa12eb9821602#107",
        Ordering::Greater,
    );
    // - exp_diff_rem == 0 second time in add_float_significands_same_prec_general
    test(
        "5.19192095203e-15",
        "0x1.761e097c5E-12#37",
        "7.4e4",
        "0x1.2E+4#5",
        RoundingMode::Nearest,
        "73728.0",
        "0x12000.00000#37",
        Ordering::Less,
    );
    // - shift <= 1 in add_float_significands_same_prec_general
    test(
        "15135.895602865542606017656527713819177465060416097749360065",
        "0x3b1f.e5463ab9b599ce49b83c7988b324dc93ce50b2ed51a18#191",
        "3.581529624499970047886732225242180736649e-8",
        "0x9.9d355ad2b99a587727da095fa3226bf0E-7#130",
        RoundingMode::Nearest,
        "15135.895602901357902262656228192686499717482223464235884113",
        "0x3b1f.e5463b5388ef7b7551e200fb30c5728e007771ed51a18#191",
        Ordering::Equal,
    );
    // - round_bit == Uninitialized fourth time in add_float_significands_same_prec_general
    // - round_bit == Uninitialized seventh time in add_float_significands_same_prec_general
    test(
        "8.63643735016344467819174862798332593462e-6",
        "0x0.000090e5374a001358c6606f968bf3813ad9#128",
        "1.84904e-8",
        "0x4.f6a6E-7#18",
        RoundingMode::Nearest,
        "8.65492771851100147411665059026359937212e-6",
        "0x0.00009134a1aa001358c6606f968bf3813ad9#128",
        Ordering::Equal,
    );
    // - round_bit == Uninitialized fifth time in add_float_significands_same_prec_general
    test(
        "2.389545997e25",
        "0x1.3c40f7bE+21#29",
        "0.078263259824284000402",
        "0x0.14090f9d6c745bc06#64",
        RoundingMode::Nearest,
        "2.389545996756557709e25",
        "0x1.3c40f7b000000000E+21#64",
        Ordering::Less,
    );
    // - round_bit == Uninitialized seventh time in add_float_significands_same_prec_general
    test(
        "5.7505515877842013577e-7",
        "0x9.a5d7d56cabed47dE-6#64",
        "1.1758894e-14",
        "0x3.4f515E-12#22",
        RoundingMode::Nearest,
        "5.7505517053731436845e-7",
        "0x9.a5d7d8bbfd3d47dE-6#64",
        Ordering::Equal,
    );
    // - x != 0 && x == mask second time in add_float_significands_same_prec_general
    // - xs_len <= out_len && following_bits == True in add_float_significands_same_prec_general
    test(
        "1.081090215247020702e-18",
        "0x1.3f14ddfe22c0634E-15#59",
        "6.3799657596147e-8",
        "0x1.12047722d26cE-6#47",
        RoundingMode::Nearest,
        "6.37996575972280156e-8",
        "0x1.12047722e65d4dcE-6#59",
        Ordering::Less,
    );
    // - shift == 0 in add_float_significands_same_prec_general
    test(
        "4.3055451539258443718732375731462554408177909736937057433067e-16",
        "0x1.f06543668e6018c20c17efed72ff6d3d65a4c5dc9db475b0E-13#192",
        "1.6388436e-15",
        "0x7.61754E-13#21",
        RoundingMode::Nearest,
        "2.0693980969049410047184094732421002104686218250305033868307e-15",
        "0x9.51da83668e6018c20c17efed72ff6d3d65a4c5dc9db475bE-13#192",
        Ordering::Equal,
    );
    // - yi != 0 second time in add_float_significands_same_prec_general
    test(
        "2.24181435676546e-16",
        "0x1.0276ae5de1e8E-13#47",
        "7.6430700039878539638425372386462404393e-36",
        "0xa.28cd4cb186f5925ddb0d1ecb9681103E-30#128",
        RoundingMode::Nearest,
        "2.24181435676546206911083333246297446029e-16",
        "0x1.0276ae5de1e80000a28cd4cb186f5926E-13#128",
        Ordering::Greater,
    );
    // - x != 0 && x == mask first time in add_float_significands_same_prec_general
    test(
        "2.1474796e9",
        "0x7.ffff00E+7#24",
        "8191.9998788833609",
        "0x1fff.fff80fffff0#54",
        RoundingMode::Nearest,
        "2147487743.9998789",
        "0x80000fff.fff810#54",
        Ordering::Greater,
    );
}

#[test]
fn add_round_fail() {
    assert_panic!(Float::one_prec(1).add_round(Float::two_prec(1), RoundingMode::Exact));
    assert_panic!(Float::one_prec(1).add_round_val_ref(&Float::two_prec(1), RoundingMode::Exact));
    assert_panic!(Float::one_prec(1).add_round_ref_val(Float::two_prec(1), RoundingMode::Exact));
    assert_panic!(Float::one_prec(1).add_round_ref_ref(&Float::two_prec(1), RoundingMode::Exact));

    assert_panic!(
        parse_hex_string("0x1.0#1").add_round(parse_hex_string("0x0.001#1"), RoundingMode::Exact)
    );
    assert_panic!(parse_hex_string("0x1.0#1")
        .add_round_val_ref(&parse_hex_string("0x0.001#1"), RoundingMode::Exact));
    assert_panic!(parse_hex_string("0x1.0#1")
        .add_round_ref_val(parse_hex_string("0x0.001#1"), RoundingMode::Exact));
    assert_panic!(parse_hex_string("0x1.0#1")
        .add_round_ref_ref(&parse_hex_string("0x0.001#1"), RoundingMode::Exact));

    assert_panic!(parse_hex_string("0x1.0000000000000000#64").add_round(
        parse_hex_string("0x1.0000000000000002#64"),
        RoundingMode::Exact
    ));
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#64").add_round_val_ref(
            &parse_hex_string("0x1.0000000000000002#64"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#64").add_round_ref_val(
            parse_hex_string("0x1.0000000000000002#64"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#64").add_round_ref_ref(
            &parse_hex_string("0x1.0000000000000002#64"),
            RoundingMode::Exact
        )
    );

    assert_panic!(parse_hex_string("0x1.0000000000000000#65").add_round(
        parse_hex_string("0x1.0000000000000001#65"),
        RoundingMode::Exact
    ));
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#65").add_round_val_ref(
            &parse_hex_string("0x1.0000000000000001#65"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#65").add_round_ref_val(
            parse_hex_string("0x1.0000000000000001#65"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.0000000000000000#65").add_round_ref_ref(
            &parse_hex_string("0x1.0000000000000001#65"),
            RoundingMode::Exact
        )
    );

    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#128").add_round(
            parse_hex_string("0x1.00000000000000000000000000000002#128"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#128").add_round_val_ref(
            &parse_hex_string("0x1.00000000000000000000000000000002#128"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#128").add_round_ref_val(
            parse_hex_string("0x1.00000000000000000000000000000002#128"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#128").add_round_ref_ref(
            &parse_hex_string("0x1.00000000000000000000000000000002#128"),
            RoundingMode::Exact
        )
    );

    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#129").add_round(
            parse_hex_string("0x1.00000000000000000000000000000003#129"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#129").add_round_val_ref(
            &parse_hex_string("0x1.00000000000000000000000000000003#129"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#129").add_round_ref_val(
            parse_hex_string("0x1.00000000000000000000000000000003#129"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.00000000000000000000000000000000#129").add_round_ref_ref(
            &parse_hex_string("0x1.00000000000000000000000000000003#129"),
            RoundingMode::Exact
        )
    );

    assert_panic!(
        parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192").add_round(
            parse_hex_string("0x1.000000000000000000000000000000000000000000000002#192"),
            RoundingMode::Exact
        )
    );
    assert_panic!(
        parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192")
            .add_round_val_ref(
                &parse_hex_string("0x1.000000000000000000000000000000000000000000000002#192"),
                RoundingMode::Exact
            )
    );
    assert_panic!(
        parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192")
            .add_round_ref_val(
                parse_hex_string("0x1.000000000000000000000000000000000000000000000002#192"),
                RoundingMode::Exact
            )
    );
    assert_panic!(
        parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192")
            .add_round_ref_ref(
                &parse_hex_string("0x1.000000000000000000000000000000000000000000000002#192"),
                RoundingMode::Exact
            )
    );

    assert_panic!(
        parse_hex_string("0x1.0#1").add_round(parse_hex_string("0x1.8#2"), RoundingMode::Exact)
    );
    assert_panic!(parse_hex_string("0x1.0#1")
        .add_round_val_ref(&parse_hex_string("0x1.8#2"), RoundingMode::Exact));
    assert_panic!(parse_hex_string("0x1.0#1")
        .add_round_ref_val(parse_hex_string("0x1.8#2"), RoundingMode::Exact));
    assert_panic!(parse_hex_string("0x1.0#1")
        .add_round_ref_ref(&parse_hex_string("0x1.8#2"), RoundingMode::Exact));

    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_round_assign(Float::two_prec(1), RoundingMode::Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_round_assign_ref(&Float::two_prec(1), RoundingMode::Exact)
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.0#1");
        x.add_round_assign(parse_hex_string("0x0.001#1"), RoundingMode::Exact)
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.0#1");
        x.add_round_assign_ref(&parse_hex_string("0x0.001#1"), RoundingMode::Exact)
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.0000000000000000#64");
        x.add_round_assign(
            parse_hex_string("0x1.0000000000000002#64"),
            RoundingMode::Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.0000000000000000#64");
        x.add_round_assign_ref(
            &parse_hex_string("0x1.0000000000000002#64"),
            RoundingMode::Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.0000000000000000#65");
        x.add_round_assign(
            parse_hex_string("0x1.0000000000000001#65"),
            RoundingMode::Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.0000000000000000#65");
        x.add_round_assign_ref(
            &parse_hex_string("0x1.0000000000000001#65"),
            RoundingMode::Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.00000000000000000000000000000000#128");
        x.add_round_assign(
            parse_hex_string("0x1.00000000000000000000000000000002#128"),
            RoundingMode::Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.00000000000000000000000000000000#128");
        x.add_round_assign_ref(
            &parse_hex_string("0x1.00000000000000000000000000000002#128"),
            RoundingMode::Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.00000000000000000000000000000000#129");
        x.add_round_assign(
            parse_hex_string("0x1.00000000000000000000000000000003#129"),
            RoundingMode::Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.00000000000000000000000000000000#129");
        x.add_round_assign_ref(
            &parse_hex_string("0x1.00000000000000000000000000000003#129"),
            RoundingMode::Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192");
        x.add_round_assign(
            parse_hex_string("0x1.000000000000000000000000000000000000000000000002#192"),
            RoundingMode::Exact,
        )
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.000000000000000000000000000000000000000000000000#192");
        x.add_round_assign_ref(
            &parse_hex_string("0x1.000000000000000000000000000000000000000000000002#192"),
            RoundingMode::Exact,
        )
    });

    assert_panic!({
        let mut x = parse_hex_string("0x1.0#1");
        x.add_round_assign(parse_hex_string("0x1.8#2"), RoundingMode::Exact)
    });
    assert_panic!({
        let mut x = parse_hex_string("0x1.0#1");
        x.add_round_assign_ref(&parse_hex_string("0x1.8#2"), RoundingMode::Exact)
    });
}

#[test]
fn test_add_prec_round() {
    let test = |s, s_hex, t, t_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (sum, o) = x.clone().add_prec_round(y.clone(), prec, rm);
        assert!(sum.is_valid());

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);
        assert_eq!(o, o_out);

        let (sum_alt, o_alt) = x.clone().add_prec_round_val_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = x.add_prec_round_ref_val(y.clone(), prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = x.add_prec_round_ref_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_prec_round_assign(y.clone(), prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_prec_round_assign_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o_out);

        let (sum_alt, o_alt) = add_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    };
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "NaN",
        "NaN",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Floor,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    // Note different behavior for Floor
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "NaN",
        "NaN",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    // Note different behavior for Floor
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Ceiling,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Down,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Up,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Nearest,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Exact,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "NaN",
        "NaN",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "Infinity",
        "Infinity",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0.0",
        "0x0.0",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-0.0",
        "-0x0.0",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123.0",
        "0x7b.0#7",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        1,
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        1,
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        1,
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        1,
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        1,
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        1,
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        1,
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        1,
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        1,
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        1,
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        1,
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        1,
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        1,
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        1,
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        1,
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        1,
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        1,
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        1,
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        1,
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        1,
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Floor,
        "4.555",
        "0x4.8e#10",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Ceiling,
        "4.56",
        "0x4.90#10",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Down,
        "4.555",
        "0x4.8e#10",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Up,
        "4.56",
        "0x4.90#10",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Nearest,
        "4.555",
        "0x4.8e#10",
        Ordering::Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Floor,
        "-1.729",
        "-0x1.ba8#10",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Ceiling,
        "-1.727",
        "-0x1.ba0#10",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Down,
        "-1.727",
        "-0x1.ba0#10",
        Ordering::Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Up,
        "-1.729",
        "-0x1.ba8#10",
        Ordering::Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Nearest,
        "-1.727",
        "-0x1.ba0#10",
        Ordering::Greater,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Floor,
        "1.727",
        "0x1.ba0#10",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Ceiling,
        "1.729",
        "0x1.ba8#10",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Down,
        "1.727",
        "0x1.ba0#10",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Up,
        "1.729",
        "0x1.ba8#10",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        RoundingMode::Nearest,
        "1.727",
        "0x1.ba0#10",
        Ordering::Less,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Floor,
        "-4.56",
        "-0x4.90#10",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Ceiling,
        "-4.555",
        "-0x4.8e#10",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Down,
        "-4.555",
        "-0x4.8e#10",
        Ordering::Greater,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Up,
        "-4.56",
        "-0x4.90#10",
        Ordering::Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        RoundingMode::Nearest,
        "-4.555",
        "-0x4.8e#10",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        1,
        RoundingMode::Floor,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        1,
        RoundingMode::Ceiling,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        1,
        RoundingMode::Down,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        1,
        RoundingMode::Up,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        1,
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        20,
        RoundingMode::Floor,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        20,
        RoundingMode::Ceiling,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        20,
        RoundingMode::Down,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        20,
        RoundingMode::Up,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        20,
        RoundingMode::Nearest,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        20,
        RoundingMode::Exact,
        "1.000244",
        "0x1.00100#20",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        10,
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        10,
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        10,
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        10,
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        10,
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1.0",
        "-0x1.0#1",
        10,
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    // - exp_diff_rem == 0 && yi == 0 first time in add_float_significands_same_prec_general
    test(
        "0.000487804412841796875000000000000000000000000000000000000000291703841149741293879771765\
        542",
        "0x0.001ff8000000000000000000000000000000000000000000007800000000001fffffffffffe#288",
        "0.000488281249999997",
        "0x0.001fffffffffffc#47",
        53,
        RoundingMode::Ceiling,
        "0.0009760856628417935",
        "0x0.003ff7ffffffffc2#53",
        Ordering::Greater,
    );
    test(
        "0.000488281249999997",
        "0x0.001fffffffffffc#47",
        "0.000487804412841796875000000000000000000000000000000000000000291703841149741293879771765\
        542",
        "0x0.001ff8000000000000000000000000000000000000000000007800000000001fffffffffffe#288",
        53,
        RoundingMode::Ceiling,
        "0.0009760856628417935",
        "0x0.003ff7ffffffffc2#53",
        Ordering::Greater,
    );
    // - round_bit != Uninitialized third time in add_float_significands_same_prec_general
    test(
        "54004143877011445683.03364332940006328475124657021",
        "0x2ed7542badc5a97b3.089cd9678077bdca8dabe52c#160",
        "8.0",
        "0x8.0#1",
        127,
        RoundingMode::Up,
        "54004143877011445691.033643329400063285",
        "0x2ed7542badc5a97bb.089cd9678077bdd0#127",
        Ordering::Greater,
    );
}

#[test]
fn add_prec_round_fail() {
    assert_panic!(Float::one_prec(1).add_prec_round(Float::two_prec(1), 0, RoundingMode::Floor));
    assert_panic!(Float::one_prec(1).add_prec_round_val_ref(
        &Float::two_prec(1),
        0,
        RoundingMode::Floor
    ));
    assert_panic!(Float::one_prec(1).add_prec_round_ref_val(
        Float::two_prec(1),
        0,
        RoundingMode::Floor
    ));
    assert_panic!(Float::one_prec(1).add_prec_round_ref_ref(
        &Float::two_prec(1),
        0,
        RoundingMode::Floor
    ));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_prec_round_assign(Float::two_prec(1), 0, RoundingMode::Floor)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_prec_round_assign_ref(&Float::two_prec(1), 0, RoundingMode::Floor)
    });

    assert_panic!(Float::one_prec(1).add_prec_round(Float::two_prec(1), 1, RoundingMode::Exact));
    assert_panic!(Float::one_prec(1).add_prec_round_val_ref(
        &Float::two_prec(1),
        1,
        RoundingMode::Exact
    ));
    assert_panic!(Float::one_prec(1).add_prec_round_ref_val(
        Float::two_prec(1),
        1,
        RoundingMode::Exact
    ));
    assert_panic!(Float::one_prec(1).add_prec_round_ref_ref(
        &Float::two_prec(1),
        1,
        RoundingMode::Exact
    ));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_prec_round_assign(Float::two_prec(1), 1, RoundingMode::Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_prec_round_assign_ref(&Float::two_prec(1), 1, RoundingMode::Exact)
    });
}

#[test]
fn test_add_rational() {
    let test = |s, s_hex, t, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let sum = x.clone() + y.clone();
        assert!(sum.is_valid());

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);

        let sum_alt = x.clone() + &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = &x + y.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = &x + &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));

        let sum_alt = y.clone() + x.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = y.clone() + &x;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = &y + x.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let sum_alt = &y + &x;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));

        let mut sum_alt = x.clone();
        sum_alt += y.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        let mut sum_alt = x.clone();
        sum_alt += &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_add_rational(
                rug::Float::exact_from(&x),
                rug::Rational::from(&y)
            ))),
            ComparableFloatRef(&sum)
        );

        let sum_alt = add_rational_prec_round_naive(
            x.clone(),
            y.clone(),
            x.significant_bits(),
            RoundingMode::Nearest,
        )
        .0;
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    };
    test("NaN", "NaN", "123", "NaN", "NaN");
    test("Infinity", "Infinity", "123", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "123", "-Infinity", "-Infinity");
    test("0.0", "0x0.0", "0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "0", "-0.0", "-0x0.0");
    test("0.0", "0x0.0", "123", "1.0e2", "0x8.0E+1#1");
    test("-0.0", "-0x0.0", "123", "1.0e2", "0x8.0E+1#1");
    test("0.0", "0x0.0", "1/3", "0.2", "0x0.4#1");
    test("-0.0", "-0x0.0", "1/3", "0.2", "0x0.4#1");
    test("123.0", "0x7b.0#7", "0", "123.0", "0x7b.0#7");

    test("1.0", "0x1.0#1", "2", "4.0", "0x4.0#1");
    test("1.0", "0x1.0#2", "2", "3.0", "0x3.0#2");
    test("1.0", "0x1.000#10", "2", "3.0", "0x3.00#10");
    test("1.0", "0x1.000#10", "1/3", "1.334", "0x1.558#10");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        "3.4749259869231266",
        "0x3.7994bfdddaf86#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        "2.8082593202564596",
        "0x2.ceea1533304da#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        "-2.8082593202564596",
        "-0x2.ceea1533304da#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        "-3.4749259869231266",
        "-0x3.7994bfdddaf86#53",
    );

    test("1.0", "0x1.0#1", "1/50000", "1.0", "0x1.0#1");
    test("1.0", "0x1.0#1", "-1", "0.0", "0x0.0");

    test(
        "1.832e180",
        "0x7.10E+149#10",
        "-1/8388607",
        "1.832e180",
        "0x7.10E+149#10",
    );
    test("1.0", "0x1.0#1", "0", "1.0", "0x1.0#1");
    // - o == Ordering::Equal in add_rational_prec_round_assign_helper
    test("1.0", "0x1.0#1", "1", "2.0", "0x2.0#1");
    // - o != Ordering::Equal in add_rational_prec_round_assign_helper
    // - t != 0 in add_rational_prec_round_assign_helper
    // - small Natural in float_can_round
    // - err0 > prec && err > prec in float_can_round
    // - s != Limb::WIDTH first time in float_can_round
    // - n == 0 in float_can_round
    // - float_can_round in add_rational_prec_round_assign_helper
    test("1.0", "0x1.0#1", "1/3", "1.0", "0x1.0#1");
    // - large Natural in float_can_round
    // - n != 0 && tmp != 0 && tmp != mask in float_can_round
    test(
        "269104312292334.303",
        "0xf4bfbaf113ee.4d8#57",
        "517543599148951977/6042448266342026218192",
        "269104312292334.303",
        "0xf4bfbaf113ee.4d8#57",
    );
    // - s == Limb::WIDTH first time in float_can_round
    test(
        "1.1595752615776271305e-33",
        "0x6.055703bef650178E-28#63",
        "-2457795567751474853961645492284796573970712506001349310379799846240055987994196472114730\
        61298054082441720799/23",
        "-1.0686067685875977626e106",
        "-0x1.2a31c100f5e98110E+88#63",
    );
    // - !float_can_round in add_rational_prec_round_assign_helper
    // - n != 0 && tmp == 0 in float_can_round
    // - n <= 0 first time in float_can_round
    // - s != Limb::WIDTH second time in float_can_round
    // - n > 0 first time in float_can_round
    // - x == 0 in float_can_round
    // - x != 0 in float_can_round
    test(
        "5.82156e33",
        "0x1.1f066E+28#20",
        "8238723/1413731881599214931",
        "5.82156e33",
        "0x1.1f066E+28#20",
    );
    // - n != 0 && tmp != 0 && tmp == mask in float_can_round
    // - n <= 0 second time in float_can_round
    // - s != Limb::WIDTH third time in float_can_round
    test(
        "1.04226364758062811487679885e63",
        "0x2.889a2dba3978ccd56c826E+52#85",
        "-3183521742267703581572109801877979/35801077870645726",
        "1.04226364758062811487679885e63",
        "0x2.889a2dba3978ccd56c826E+52#85",
    );
    // - s == Limb::WIDTH second time in float_can_round
    // - n > 0 second time in float_can_round
    // - x == Limb::MAX in float_can_round
    test(
        "5.3586423373910357e63",
        "0xd.06b0f9ea88b7aE+52#55",
        "-808694/11557016618486698036897716112870533564271922504817764249868611771",
        "5.3586423373910357e63",
        "0xd.06b0f9ea88b7aE+52#55",
    );
    // - x != Limb::MAX in float_can_round
    test(
        "4.9046e10",
        "0xb.6b5eE+8#19",
        "-182971083215/47776579472461276067598384651239241670113870043",
        "4.9046e10",
        "0xb.6b5eE+8#19",
    );
    // - s == Limb::WIDTH third time in float_can_round
    test(
        "2.515940926449112e24",
        "0x2.14c56e507323d0E+20#55",
        "-3161/286335616056113777",
        "2.515940926449112e24",
        "0x2.14c56e507323d0E+20#55",
    );
    // - err0 <= prec || err <= prec in float_can_round
    test(
        "5.96046446e-8",
        "0xf.ffffffE-7#28",
        "-255/4261412864",
        "-2.34664179e-10",
        "-0x1.0204182E-8#28",
    );
    // - t == 0 in add_rational_prec_round_assign_helper
    test(
        "64.0",
        "0x40.00000#24",
        "-215679573337202053366254388918461596342975394375083745820919489101824/336999333319768176\
        9435117813341997219028918889483691361889027096575",
        "-3.7241676e-9",
        "-0xf.fec40E-8#24",
    );
}

#[test]
fn test_add_rational_prec() {
    let test = |s, s_hex, t, prec, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (sum, o) = x.clone().add_rational_prec(y.clone(), prec);
        assert!(sum.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);

        let (sum_alt, o_alt) = x.clone().add_rational_prec_val_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.add_rational_prec_ref_val(y.clone(), prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.add_rational_prec_ref_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_rational_prec_assign(y.clone(), prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_rational_prec_assign_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) =
            add_rational_prec_round_naive(x.clone(), y.clone(), prec, RoundingMode::Nearest);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    };
    test("NaN", "NaN", "123", 1, "NaN", "NaN", Ordering::Equal);
    test(
        "Infinity",
        "Infinity",
        "123",
        1,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test("0.0", "0x0.0", "0", 1, "0.0", "0x0.0", Ordering::Equal);
    test("-0.0", "-0x0.0", "0", 1, "-0.0", "-0x0.0", Ordering::Equal);
    test(
        "0.0",
        "0x0.0",
        "123",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test("0.0", "0x0.0", "1/3", 1, "0.2", "0x0.4#1", Ordering::Less);
    test("-0.0", "-0x0.0", "1/3", 1, "0.2", "0x0.4#1", Ordering::Less);
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test("1.0", "0x1.0#1", "2", 2, "3.0", "0x3.0#2", Ordering::Equal);
    test(
        "1.0",
        "0x1.0#2",
        "2",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test("1.0", "0x1.0#2", "2", 2, "3.0", "0x3.0#2", Ordering::Equal);
    test(
        "1.0",
        "0x1.000#10",
        "2",
        1,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2",
        2,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        100,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        "3.477",
        "0x3.7a#10",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        "2.809",
        "0x2.cf#10",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        "-2.809",
        "-0x2.cf#10",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        "-3.477",
        "-0x3.7a#10",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        10,
        "1.0",
        "0x1.000#10",
        Ordering::Less,
    );
    test("1.0", "0x1.0#1", "-1", 10, "0.0", "0x0.0", Ordering::Equal);

    test(
        "4.547473508864641189582899002890190173e-13",
        "0x8.00000000000000003ffff000000000E-11#123",
        "-4194304/9223372019742015487",
        3,
        "-8.0e-22",
        "-0x4.0E-18#3",
        Ordering::Less,
    );
}

#[test]
fn add_rational_prec_fail() {
    assert_panic!(Float::NAN.add_rational_prec(Rational::ZERO, 0));
    assert_panic!(Float::NAN.add_rational_prec_val_ref(&Rational::ZERO, 0));
    assert_panic!(Float::NAN.add_rational_prec_ref_val(Rational::ZERO, 0));
    assert_panic!(Float::NAN.add_rational_prec_ref_ref(&Rational::ZERO, 0));
    assert_panic!({
        let mut x = Float::NAN;
        x.add_rational_prec_assign(Rational::ZERO, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.add_rational_prec_assign_ref(&Rational::ZERO, 0)
    });
}

#[test]
fn test_add_rational_round() {
    let test = |s, s_hex, t, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (sum, o) = x.clone().add_rational_round(y.clone(), rm);
        assert!(sum.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);

        let (sum_alt, o_alt) = x.clone().add_rational_round_val_ref(&y, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.add_rational_round_ref_val(y.clone(), rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.add_rational_round_ref_ref(&y, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_rational_round_assign(y.clone(), rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_rational_round_assign_ref(&y, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sum, rug_o) = rug_add_rational_round(
                rug::Float::exact_from(&x),
                rug::Rational::exact_from(&y),
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sum)),
                ComparableFloatRef(&sum)
            );
            assert_eq!(rug_o, o);
        }

        let (sum_alt, o_alt) =
            add_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    };
    test(
        "NaN",
        "NaN",
        "123",
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123",
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123",
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123",
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123",
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123",
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "123",
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123",
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123",
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123",
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123",
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123",
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "123",
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "0",
        RoundingMode::Floor,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "0",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        RoundingMode::Ceiling,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        RoundingMode::Down,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        RoundingMode::Up,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        RoundingMode::Nearest,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        RoundingMode::Exact,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "123",
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "123",
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "123",
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "123",
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "123",
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "-0.0",
        "-0x0.0",
        "123",
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123",
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123",
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123",
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123",
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "0",
        RoundingMode::Floor,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        RoundingMode::Ceiling,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        RoundingMode::Down,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        RoundingMode::Up,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        RoundingMode::Nearest,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        RoundingMode::Exact,
        "123.0",
        "0x7b.0#7",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "1/3",
        RoundingMode::Floor,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "1/3",
        RoundingMode::Ceiling,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "1/3",
        RoundingMode::Down,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "1/3",
        RoundingMode::Up,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "1/3",
        RoundingMode::Nearest,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );

    test(
        "-0.0",
        "-0x0.0",
        "1/3",
        RoundingMode::Floor,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "1/3",
        RoundingMode::Ceiling,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "1/3",
        RoundingMode::Down,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "1/3",
        RoundingMode::Up,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "1/3",
        RoundingMode::Nearest,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2",
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2",
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2",
        RoundingMode::Ceiling,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2",
        RoundingMode::Down,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2",
        RoundingMode::Up,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2",
        RoundingMode::Nearest,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2",
        RoundingMode::Exact,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "2",
        RoundingMode::Floor,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2",
        RoundingMode::Ceiling,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2",
        RoundingMode::Down,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2",
        RoundingMode::Up,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2",
        RoundingMode::Nearest,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2",
        RoundingMode::Exact,
        "3.0",
        "0x3.00#10",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        RoundingMode::Floor,
        "1.332",
        "0x1.550#10",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        RoundingMode::Ceiling,
        "1.334",
        "0x1.558#10",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        RoundingMode::Down,
        "1.332",
        "0x1.550#10",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        RoundingMode::Up,
        "1.334",
        "0x1.558#10",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        RoundingMode::Nearest,
        "1.334",
        "0x1.558#10",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        RoundingMode::Floor,
        "1.333333333333333333333333333332",
        "0x1.5555555555555555555555554#100",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        RoundingMode::Ceiling,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        RoundingMode::Down,
        "1.333333333333333333333333333332",
        "0x1.5555555555555555555555554#100",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        RoundingMode::Up,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1/3",
        RoundingMode::Nearest,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Floor,
        "3.4749259869231262",
        "0x3.7994bfdddaf84#53",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Ceiling,
        "3.4749259869231266",
        "0x3.7994bfdddaf86#53",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Down,
        "3.4749259869231262",
        "0x3.7994bfdddaf84#53",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Up,
        "3.4749259869231266",
        "0x3.7994bfdddaf86#53",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Nearest,
        "3.4749259869231266",
        "0x3.7994bfdddaf86#53",
        Ordering::Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Floor,
        "2.8082593202564596",
        "0x2.ceea1533304da#53",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Ceiling,
        "2.8082593202564601",
        "0x2.ceea1533304dc#53",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Down,
        "2.8082593202564596",
        "0x2.ceea1533304da#53",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Up,
        "2.8082593202564601",
        "0x2.ceea1533304dc#53",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Nearest,
        "2.8082593202564596",
        "0x2.ceea1533304da#53",
        Ordering::Less,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Floor,
        "-2.8082593202564601",
        "-0x2.ceea1533304dc#53",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Ceiling,
        "-2.8082593202564596",
        "-0x2.ceea1533304da#53",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Down,
        "-2.8082593202564596",
        "-0x2.ceea1533304da#53",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Up,
        "-2.8082593202564601",
        "-0x2.ceea1533304dc#53",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        RoundingMode::Nearest,
        "-2.8082593202564596",
        "-0x2.ceea1533304da#53",
        Ordering::Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Floor,
        "-3.4749259869231266",
        "-0x3.7994bfdddaf86#53",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Ceiling,
        "-3.4749259869231262",
        "-0x3.7994bfdddaf84#53",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Down,
        "-3.4749259869231262",
        "-0x3.7994bfdddaf84#53",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Up,
        "-3.4749259869231266",
        "-0x3.7994bfdddaf86#53",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        RoundingMode::Nearest,
        "-3.4749259869231266",
        "-0x3.7994bfdddaf86#53",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        RoundingMode::Floor,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        RoundingMode::Ceiling,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        RoundingMode::Down,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        RoundingMode::Up,
        "2.0",
        "0x2.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-1",
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1",
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1",
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1",
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1",
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1",
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "1.832e180",
        "0x7.10E+149#10",
        "-1/8388607",
        RoundingMode::Floor,
        "1.83e180",
        "0x7.0eE+149#10",
        Ordering::Less,
    );
    test(
        "1.832e180",
        "0x7.10E+149#10",
        "-1/8388607",
        RoundingMode::Ceiling,
        "1.832e180",
        "0x7.10E+149#10",
        Ordering::Greater,
    );
    test(
        "1.832e180",
        "0x7.10E+149#10",
        "-1/8388607",
        RoundingMode::Down,
        "1.83e180",
        "0x7.0eE+149#10",
        Ordering::Less,
    );
    test(
        "1.832e180",
        "0x7.10E+149#10",
        "-1/8388607",
        RoundingMode::Up,
        "1.832e180",
        "0x7.10E+149#10",
        Ordering::Greater,
    );
    test(
        "1.832e180",
        "0x7.10E+149#10",
        "-1/8388607",
        RoundingMode::Nearest,
        "1.832e180",
        "0x7.10E+149#10",
        Ordering::Greater,
    );
    test(
        "1.832e180",
        "0x7.10E+149#10",
        "-1/8388607",
        RoundingMode::Nearest,
        "1.832e180",
        "0x7.10E+149#10",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0",
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#1",
        Ordering::Equal,
    );
    // - o == Ordering::Equal in add_rational_prec_round_assign_helper
    test(
        "1.0",
        "0x1.0#1",
        "1",
        RoundingMode::Nearest,
        "2.0",
        "0x2.0#1",
        Ordering::Equal,
    );
    // - o != Ordering::Equal in add_rational_prec_round_assign_helper
    // - t != 0 in add_rational_prec_round_assign_helper
    // - small Natural in float_can_round
    // - err0 > prec && err > prec in float_can_round
    // - s != Limb::WIDTH first time in float_can_round
    // - n == 0 in float_can_round
    // - float_can_round in add_rational_prec_round_assign_helper
    test(
        "1.0",
        "0x1.0#1",
        "1/3",
        RoundingMode::Nearest,
        "1.0",
        "0x1.0#1",
        Ordering::Less,
    );
    // - large Natural in float_can_round
    // - n != 0 && tmp != 0 && tmp != mask in float_can_round
    test(
        "269104312292334.303",
        "0xf4bfbaf113ee.4d8#57",
        "517543599148951977/6042448266342026218192",
        RoundingMode::Nearest,
        "269104312292334.303",
        "0xf4bfbaf113ee.4d8#57",
        Ordering::Less,
    );
    // - s == Limb::WIDTH first time in float_can_round
    test(
        "1.1595752615776271305e-33",
        "0x6.055703bef650178E-28#63",
        "-2457795567751474853961645492284796573970712506001349310379799846240055987994196472114730\
        61298054082441720799/23",
        RoundingMode::Nearest,
        "-1.0686067685875977626e106",
        "-0x1.2a31c100f5e98110E+88#63",
        Ordering::Less,
    );
    // - !float_can_round in add_rational_prec_round_assign_helper
    // - n != 0 && tmp == 0 in float_can_round
    // - n <= 0 first time in float_can_round
    // - s != Limb::WIDTH second time in float_can_round
    // - n > 0 first time in float_can_round
    // - x == 0 in float_can_round
    // - x != 0 in float_can_round
    test(
        "5.82156e33",
        "0x1.1f066E+28#20",
        "8238723/1413731881599214931",
        RoundingMode::Nearest,
        "5.82156e33",
        "0x1.1f066E+28#20",
        Ordering::Less,
    );
    // - n != 0 && tmp != 0 && tmp == mask in float_can_round
    // - n <= 0 second time in float_can_round
    // - s != Limb::WIDTH third time in float_can_round
    test(
        "1.04226364758062811487679885e63",
        "0x2.889a2dba3978ccd56c826E+52#85",
        "-3183521742267703581572109801877979/35801077870645726",
        RoundingMode::Nearest,
        "1.04226364758062811487679885e63",
        "0x2.889a2dba3978ccd56c826E+52#85",
        Ordering::Greater,
    );
    // - s == Limb::WIDTH second time in float_can_round
    // - n > 0 second time in float_can_round
    // - x == Limb::MAX in float_can_round
    test(
        "5.3586423373910357e63",
        "0xd.06b0f9ea88b7aE+52#55",
        "-808694/11557016618486698036897716112870533564271922504817764249868611771",
        RoundingMode::Nearest,
        "5.3586423373910357e63",
        "0xd.06b0f9ea88b7aE+52#55",
        Ordering::Greater,
    );
    // - x != Limb::MAX in float_can_round
    test(
        "4.9046e10",
        "0xb.6b5eE+8#19",
        "-182971083215/47776579472461276067598384651239241670113870043",
        RoundingMode::Nearest,
        "4.9046e10",
        "0xb.6b5eE+8#19",
        Ordering::Greater,
    );
    // - s == Limb::WIDTH third time in float_can_round
    test(
        "2.515940926449112e24",
        "0x2.14c56e507323d0E+20#55",
        "-3161/286335616056113777",
        RoundingMode::Nearest,
        "2.515940926449112e24",
        "0x2.14c56e507323d0E+20#55",
        Ordering::Greater,
    );
    // - err0 <= prec || err <= prec in float_can_round
    test(
        "5.96046446e-8",
        "0xf.ffffffE-7#28",
        "-255/4261412864",
        RoundingMode::Nearest,
        "-2.34664179e-10",
        "-0x1.0204182E-8#28",
        Ordering::Less,
    );
    // - t == 0 in add_rational_prec_round_assign_helper
    test(
        "64.0",
        "0x40.00000#24",
        "-215679573337202053366254388918461596342975394375083745820919489101824/336999333319768176\
        9435117813341997219028918889483691361889027096575",
        RoundingMode::Nearest,
        "-3.7241676e-9",
        "-0xf.fec40E-8#24",
        Ordering::Greater,
    );
}

#[test]
fn add_rational_round_fail() {
    assert_panic!(Float::one_prec(1)
        .add_rational_round(Rational::from_unsigneds(1u32, 3), RoundingMode::Exact));
    assert_panic!(Float::one_prec(1)
        .add_rational_round_val_ref(&Rational::from_unsigneds(1u32, 3), RoundingMode::Exact));
    assert_panic!(Float::one_prec(1)
        .add_rational_round_ref_val(Rational::from_unsigneds(1u32, 3), RoundingMode::Exact));
    assert_panic!(Float::one_prec(1)
        .add_rational_round_ref_ref(&Rational::from_unsigneds(1u32, 3), RoundingMode::Exact));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_rational_round_assign(Rational::from_unsigneds(1u32, 3), RoundingMode::Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_rational_round_assign_ref(&Rational::from_unsigneds(1u32, 3), RoundingMode::Exact)
    });
}

#[test]
fn test_add_rational_prec_round() {
    let test = |s, s_hex, t, prec, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(t).unwrap();

        let (sum, o) = x.clone().add_rational_prec_round(y.clone(), prec, rm);
        assert!(sum.is_valid());
        assert_eq!(o, o_out);

        assert_eq!(sum.to_string(), out);
        assert_eq!(to_hex_string(&sum), out_hex);

        let (sum_alt, o_alt) = x.clone().add_rational_prec_round_val_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.add_rational_prec_round_ref_val(y.clone(), prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.add_rational_prec_round_ref_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_rational_prec_round_assign(y.clone(), prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let mut sum_alt = x.clone();
        let o_alt = sum_alt.add_rational_prec_round_assign_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&sum_alt));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = add_rational_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
    };
    test(
        "NaN",
        "NaN",
        "123",
        1,
        RoundingMode::Floor,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123",
        1,
        RoundingMode::Ceiling,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123",
        1,
        RoundingMode::Down,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123",
        1,
        RoundingMode::Up,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123",
        1,
        RoundingMode::Nearest,
        "NaN",
        "NaN",
        Ordering::Equal,
    );
    test(
        "NaN",
        "NaN",
        "123",
        1,
        RoundingMode::Exact,
        "NaN",
        "NaN",
        Ordering::Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "123",
        1,
        RoundingMode::Floor,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123",
        1,
        RoundingMode::Ceiling,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123",
        1,
        RoundingMode::Down,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123",
        1,
        RoundingMode::Up,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123",
        1,
        RoundingMode::Nearest,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "123",
        1,
        RoundingMode::Exact,
        "Infinity",
        "Infinity",
        Ordering::Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        RoundingMode::Floor,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        RoundingMode::Ceiling,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        RoundingMode::Down,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        RoundingMode::Up,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        RoundingMode::Nearest,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123",
        1,
        RoundingMode::Exact,
        "-Infinity",
        "-Infinity",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "0",
        1,
        RoundingMode::Floor,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        1,
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        1,
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        1,
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        1,
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "0",
        1,
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "0",
        1,
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        1,
        RoundingMode::Ceiling,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        1,
        RoundingMode::Down,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        1,
        RoundingMode::Up,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        1,
        RoundingMode::Nearest,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "0",
        1,
        RoundingMode::Exact,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "123",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "123",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "123",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "123",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "123",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "-0.0",
        "-0x0.0",
        "123",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "123",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "0.0",
        "0x0.0",
        "1/3",
        1,
        RoundingMode::Floor,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "1/3",
        1,
        RoundingMode::Ceiling,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "1/3",
        1,
        RoundingMode::Down,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "0.0",
        "0x0.0",
        "1/3",
        1,
        RoundingMode::Up,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "0.0",
        "0x0.0",
        "1/3",
        1,
        RoundingMode::Floor,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );

    test(
        "-0.0",
        "-0x0.0",
        "1/3",
        1,
        RoundingMode::Floor,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "1/3",
        1,
        RoundingMode::Ceiling,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "1/3",
        1,
        RoundingMode::Down,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );
    test(
        "-0.0",
        "-0x0.0",
        "1/3",
        1,
        RoundingMode::Up,
        "0.5",
        "0x0.8#1",
        Ordering::Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        "1/3",
        1,
        RoundingMode::Floor,
        "0.2",
        "0x0.4#1",
        Ordering::Less,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        RoundingMode::Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        RoundingMode::Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        RoundingMode::Down,
        "6.0e1",
        "0x4.0E+1#1",
        Ordering::Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        RoundingMode::Up,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "0",
        1,
        RoundingMode::Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2",
        1,
        RoundingMode::Floor,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        1,
        RoundingMode::Ceiling,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        1,
        RoundingMode::Down,
        "2.0",
        "0x2.0#1",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        1,
        RoundingMode::Up,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        1,
        RoundingMode::Nearest,
        "4.0",
        "0x4.0#1",
        Ordering::Greater,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2",
        2,
        RoundingMode::Floor,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        2,
        RoundingMode::Ceiling,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        2,
        RoundingMode::Down,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        2,
        RoundingMode::Up,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        2,
        RoundingMode::Nearest,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2",
        2,
        RoundingMode::Exact,
        "3.0",
        "0x3.0#2",
        Ordering::Equal,
    );

    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        100,
        RoundingMode::Floor,
        "1.333333333333333333333333333332",
        "0x1.5555555555555555555555554#100",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        100,
        RoundingMode::Ceiling,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        100,
        RoundingMode::Down,
        "1.333333333333333333333333333332",
        "0x1.5555555555555555555555554#100",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        100,
        RoundingMode::Up,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "1/3",
        100,
        RoundingMode::Nearest,
        "1.333333333333333333333333333334",
        "0x1.5555555555555555555555556#100",
        Ordering::Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Floor,
        "3.473",
        "0x3.79#10",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Ceiling,
        "3.477",
        "0x3.7a#10",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Down,
        "3.473",
        "0x3.79#10",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Up,
        "3.477",
        "0x3.7a#10",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Nearest,
        "3.477",
        "0x3.7a#10",
        Ordering::Greater,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Floor,
        "2.805",
        "0x2.ce#10",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Ceiling,
        "2.809",
        "0x2.cf#10",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Down,
        "2.805",
        "0x2.ce#10",
        Ordering::Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Up,
        "2.809",
        "0x2.cf#10",
        Ordering::Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Nearest,
        "2.809",
        "0x2.cf#10",
        Ordering::Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Floor,
        "-2.809",
        "-0x2.cf#10",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Ceiling,
        "-2.805",
        "-0x2.ce#10",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Down,
        "-2.805",
        "-0x2.ce#10",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Up,
        "-2.809",
        "-0x2.cf#10",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1/3",
        10,
        RoundingMode::Nearest,
        "-2.809",
        "-0x2.cf#10",
        Ordering::Less,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Floor,
        "-3.477",
        "-0x3.7a#10",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Ceiling,
        "-3.473",
        "-0x3.79#10",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Down,
        "-3.473",
        "-0x3.79#10",
        Ordering::Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Up,
        "-3.477",
        "-0x3.7a#10",
        Ordering::Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        10,
        RoundingMode::Nearest,
        "-3.477",
        "-0x3.7a#10",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        10,
        RoundingMode::Floor,
        "1.0",
        "0x1.000#10",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        10,
        RoundingMode::Ceiling,
        "1.002",
        "0x1.008#10",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        10,
        RoundingMode::Down,
        "1.0",
        "0x1.000#10",
        Ordering::Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        10,
        RoundingMode::Up,
        "1.002",
        "0x1.008#10",
        Ordering::Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1/50000",
        10,
        RoundingMode::Nearest,
        "1.0",
        "0x1.000#10",
        Ordering::Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "-1",
        10,
        RoundingMode::Floor,
        "-0.0",
        "-0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1",
        10,
        RoundingMode::Ceiling,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1",
        10,
        RoundingMode::Down,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1",
        10,
        RoundingMode::Up,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1",
        10,
        RoundingMode::Nearest,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "-1",
        10,
        RoundingMode::Exact,
        "0.0",
        "0x0.0",
        Ordering::Equal,
    );
}

#[test]
fn add_rational_prec_round_fail() {
    assert_panic!(Float::one_prec(1).add_rational_prec_round(
        Rational::from_unsigneds(5u32, 8),
        1,
        RoundingMode::Exact
    ));
    assert_panic!(Float::one_prec(1).add_rational_prec_round_val_ref(
        &Rational::from_unsigneds(5u32, 8),
        1,
        RoundingMode::Exact
    ));
    assert_panic!(Float::one_prec(1).add_rational_prec_round_ref_val(
        Rational::from_unsigneds(5u32, 8),
        1,
        RoundingMode::Exact
    ));
    assert_panic!(Float::one_prec(1).add_rational_prec_round_ref_ref(
        &Rational::from_unsigneds(5u32, 8),
        1,
        RoundingMode::Exact
    ));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_rational_prec_round_assign(Rational::from_unsigneds(5u32, 8), 1, RoundingMode::Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.add_rational_prec_round_assign_ref(
            &Rational::from_unsigneds(5u32, 8),
            1,
            RoundingMode::Exact,
        )
    });
}

#[test]
fn add_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(|(x, y, prec, rm)| {
        let (sum, o) = x.clone().add_prec_round(y.clone(), prec, rm);
        assert!(sum.is_valid());
        let (sum_alt, o_alt) = x.clone().add_prec_round_val_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
        let (sum_alt, o_alt) = x.add_prec_round_ref_val(y.clone(), prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
        let (sum_alt, o_alt) = x.add_prec_round_ref_ref(&y, prec, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.add_prec_round_assign(y.clone(), prec, rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.add_prec_round_assign_ref(&y, prec, rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = add_prec_round_naive(x.clone(), y.clone(), prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let r_sum = if sum.is_finite() {
            if sum.is_normal() {
                assert_eq!(sum.get_prec(), Some(prec));
            }
            let r_sum = Rational::exact_from(&x) + Rational::exact_from(&y);
            assert_eq!(sum.partial_cmp(&r_sum), Some(o));
            if o == Ordering::Less {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if o == Ordering::Greater {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }
            Some(r_sum)
        } else {
            assert_eq!(o, Ordering::Equal);
            None
        };

        match (r_sum.is_some() && *r_sum.as_ref().unwrap() >= 0u32, rm) {
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }

        let (sum_alt, o_alt) = y.add_prec_round_ref_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.sub_prec_round_ref_val(-&y, prec, rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let (mut sum_alt, mut o_alt) = (-&x).sub_prec_round_val_ref(&y, prec, -rm);
        sum_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloat(sum_alt.abs_negative_zero()),
            ComparableFloat(sum.abs_negative_zero_ref())
        );
        assert_eq!(o_alt, o);

        let (mut sum_alt, mut o_alt) = (-x).add_prec_round(-y, prec, -rm);
        sum_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(
            ComparableFloat(sum_alt.abs_negative_zero()),
            ComparableFloat(sum.abs_negative_zero())
        );
        assert_eq!(o_alt, o);
    });

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (sum, o) = x.add_prec_round_ref_val(Float::NAN, prec, rm);
        assert!(sum.is_nan());
        assert_eq!(o, Ordering::Equal);

        let (sum, o) = Float::NAN.add_prec_round_val_ref(&x, prec, rm);
        assert!(sum.is_nan());
        assert_eq!(o, Ordering::Equal);

        if !x.is_nan() {
            if x != Float::NEGATIVE_INFINITY {
                assert_eq!(
                    x.add_prec_round_ref_val(Float::INFINITY, prec, rm),
                    (Float::INFINITY, Ordering::Equal)
                );
                assert_eq!(
                    Float::INFINITY.add_prec_round_val_ref(&x, prec, rm),
                    (Float::INFINITY, Ordering::Equal)
                );
            }
            if x != Float::INFINITY {
                assert_eq!(
                    x.add_prec_round_ref_val(Float::NEGATIVE_INFINITY, prec, rm),
                    (Float::NEGATIVE_INFINITY, Ordering::Equal)
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY.add_prec_round_val_ref(&x, prec, rm),
                    (Float::NEGATIVE_INFINITY, Ordering::Equal)
                );
            }
        }
        if !x.is_negative_zero() {
            let (sum, o) = x.add_prec_round_ref_val(Float::ZERO, prec, rm);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);

            let (sum, o) = Float::ZERO.add_prec_round_val_ref(&x, prec, rm);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);
        }
        if rm != RoundingMode::Floor || !x.is_positive_zero() {
            let (sum, o) = x.add_prec_round_ref_val(Float::NEGATIVE_ZERO, prec, rm);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);

            let (sum, o) = Float::NEGATIVE_ZERO.add_prec_round_val_ref(&x, prec, rm);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);
        }
    });
}

#[test]
fn add_prec_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        let (sum, o) = x.clone().add_prec(y.clone(), prec);
        assert!(sum.is_valid());
        let (sum_alt, o_alt) = x.clone().add_prec_val_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
        let (sum_alt, o_alt) = x.add_prec_ref_val(y.clone(), prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
        let (sum_alt, o_alt) = x.add_prec_ref_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.add_prec_assign(y.clone(), prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.add_prec_assign_ref(&y, prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) =
            add_prec_round_naive(x.clone(), y.clone(), prec, RoundingMode::Nearest);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.add_prec_round_ref_ref(&y, prec, RoundingMode::Nearest);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        if sum.is_finite() {
            if sum.is_normal() {
                assert_eq!(sum.get_prec(), Some(prec));
            }
            let r_sum = Rational::exact_from(&x) + Rational::exact_from(&y);
            assert_eq!(sum.partial_cmp(&r_sum), Some(o));
            if o == Ordering::Less {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if o == Ordering::Greater {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }
        } else {
            assert_eq!(o, Ordering::Equal);
        }

        let (sum_alt, o_alt) = y.add_prec_ref_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.sub_prec_ref_val(-&y, prec);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        if (x != 0u32 && y != 0u32) || (x.is_sign_positive() && y.is_sign_positive()) {
            let (mut sum_alt, mut o_alt) = (-&x).sub_prec_val_ref(&y, prec);
            sum_alt.neg_assign();
            sum_alt.abs_negative_zero_assign();
            o_alt = o_alt.reverse();
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);

            let (mut sum_alt, mut o_alt) = (-x).add_prec(-y, prec);
            sum_alt.neg_assign();
            sum_alt.abs_negative_zero_assign();
            o_alt = o_alt.reverse();
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);
        }
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (sum, o) = x.add_prec_ref_val(Float::NAN, prec);
        assert!(sum.is_nan());
        assert_eq!(o, Ordering::Equal);

        let (sum, o) = Float::NAN.add_prec_val_ref(&x, prec);
        assert!(sum.is_nan());
        assert_eq!(o, Ordering::Equal);

        if !x.is_nan() {
            if x != Float::NEGATIVE_INFINITY {
                assert_eq!(
                    x.add_prec_ref_val(Float::INFINITY, prec),
                    (Float::INFINITY, Ordering::Equal)
                );
                assert_eq!(
                    Float::INFINITY.add_prec_val_ref(&x, prec),
                    (Float::INFINITY, Ordering::Equal)
                );
            }
            if x != Float::INFINITY {
                assert_eq!(
                    x.add_prec_ref_val(Float::NEGATIVE_INFINITY, prec),
                    (Float::NEGATIVE_INFINITY, Ordering::Equal)
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY.add_prec_val_ref(&x, prec),
                    (Float::NEGATIVE_INFINITY, Ordering::Equal)
                );
            }
        }
        if !x.is_negative_zero() {
            let (sum, o) = x.add_prec_ref_val(Float::ZERO, prec);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec(prec);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);

            let (sum, o) = Float::ZERO.add_prec_val_ref(&x, prec);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec(prec);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);
        }
        let (sum, o) = x.add_prec_ref_val(Float::NEGATIVE_ZERO, prec);
        let mut sum_alt = x.clone();
        let o_alt = sum_alt.set_prec(prec);
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);

        let (sum, o) = Float::NEGATIVE_ZERO.add_prec_val_ref(&x, prec);
        let mut sum_alt = x.clone();
        let o_alt = sum_alt.set_prec(prec);
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);
    });
}

fn add_round_properties_helper(x: Float, y: Float, rm: RoundingMode) {
    let (sum, o) = x.clone().add_round(y.clone(), rm);
    assert!(sum.is_valid());
    let (sum_alt, o_alt) = x.clone().add_round_val_ref(&y, rm);
    assert!(sum_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let (sum_alt, o_alt) = x.add_round_ref_val(y.clone(), rm);
    assert!(sum_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let (sum_alt, o_alt) = x.add_round_ref_ref(&y, rm);
    assert!(sum_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_round_assign(y.clone(), rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.add_round_assign_ref(&y, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let (sum_alt, o_alt) = add_prec_round_naive(
        x.clone(),
        y.clone(),
        max(x.significant_bits(), y.significant_bits()),
        rm,
    );
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);
    let (sum_alt, o_alt) =
        x.add_prec_round_ref_ref(&y, max(x.significant_bits(), y.significant_bits()), rm);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    assert_eq!(o_alt, o);

    let r_sum = if sum.is_finite() {
        if x.is_normal() && y.is_normal() && sum.is_normal() {
            assert_eq!(
                sum.get_prec(),
                Some(max(x.get_prec().unwrap(), y.get_prec().unwrap()))
            );
        }
        let r_sum = Rational::exact_from(&x) + Rational::exact_from(&y);
        assert_eq!(sum.partial_cmp(&r_sum), Some(o));
        if o == Ordering::Less {
            let mut next = sum.clone();
            next.increment();
            assert!(next > r_sum);
        } else if o == Ordering::Greater {
            let mut next = sum.clone();
            next.decrement();
            assert!(next < r_sum);
        }
        Some(r_sum)
    } else {
        assert_eq!(o, Ordering::Equal);
        None
    };

    match (r_sum.is_some() && *r_sum.as_ref().unwrap() >= 0u32, rm) {
        (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
            assert_ne!(o, Ordering::Greater)
        }
        (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
            assert_ne!(o, Ordering::Less)
        }
        (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
        _ => {}
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_sum, rug_o) =
            rug_add_round(rug::Float::exact_from(&x), rug::Float::exact_from(&y), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum)),
            ComparableFloatRef(&sum),
        );
        assert_eq!(rug_o, o);
    }

    let (sum_alt, o_alt) = y.add_round_ref_ref(&x, rm);
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let (sum_alt, o_alt) = x.sub_round_ref_val(-&y, rm);
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let (mut sum_alt, mut o_alt) = (-&x).sub_round_val_ref(&y, -rm);
    sum_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloat(sum_alt.abs_negative_zero()),
        ComparableFloat(sum.abs_negative_zero_ref())
    );

    let (mut sum_alt, mut o_alt) = (-x).add_round(-y, -rm);
    sum_alt.neg_assign();
    o_alt = o_alt.reverse();
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloat(sum_alt.abs_negative_zero()),
        ComparableFloat(sum.abs_negative_zero())
    );
}

#[test]
fn add_round_properties() {
    float_float_rounding_mode_triple_gen_var_1().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_4().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_5().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_6().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_7().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_8().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_9().test_properties(|(x, y, rm)| {
        add_round_properties_helper(x, y, rm);
    });

    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        let (sum, o) = x.add_round_ref_val(Float::NAN, rm);
        assert!(sum.is_nan());
        assert_eq!(o, Ordering::Equal);

        let (sum, o) = Float::NAN.add_round_val_ref(&x, rm);
        assert!(sum.is_nan());
        assert_eq!(o, Ordering::Equal);

        if !x.is_nan() {
            if x != Float::NEGATIVE_INFINITY {
                assert_eq!(
                    x.add_round_ref_val(Float::INFINITY, rm),
                    (Float::INFINITY, Ordering::Equal)
                );
                assert_eq!(
                    Float::INFINITY.add_round_val_ref(&x, rm),
                    (Float::INFINITY, Ordering::Equal)
                );
            }
            if x != Float::INFINITY {
                assert_eq!(
                    x.add_round_ref_val(Float::NEGATIVE_INFINITY, rm),
                    (Float::NEGATIVE_INFINITY, Ordering::Equal)
                );
                assert_eq!(
                    Float::NEGATIVE_INFINITY.add_round_val_ref(&x, rm),
                    (Float::NEGATIVE_INFINITY, Ordering::Equal)
                );
            }
        }
        if !x.is_negative_zero() {
            let (sum, o) = x.add_round_ref_val(Float::ZERO, rm);
            assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&x));
            assert_eq!(o, Ordering::Equal);
            let (sum, o) = Float::ZERO.add_round_val_ref(&x, rm);
            assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&x));
            assert_eq!(o, Ordering::Equal);
        }
        if rm != RoundingMode::Floor || !x.is_positive_zero() {
            let (sum, o) = x.add_round_ref_val(Float::NEGATIVE_ZERO, rm);
            assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&x));
            assert_eq!(o, Ordering::Equal);
            let (sum, o) = Float::NEGATIVE_ZERO.add_round_val_ref(&x, rm);
            assert_eq!(ComparableFloatRef(&sum), ComparableFloatRef(&x));
            assert_eq!(o, Ordering::Equal);
        }
    });
}

#[allow(clippy::type_repetition_in_bounds)]
fn add_properties_helper_1<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let sum_1 = x + y;
        let sum_2 = emulate_primitive_float_fn_2(|x, y, prec| x.add_prec(y, prec).0, x, y);
        assert_eq!(NiceFloat(sum_1), NiceFloat(sum_2));
    });
}

#[allow(clippy::needless_pass_by_value)]
fn add_properties_helper_2(x: Float, y: Float) {
    let sum = x.clone() + y.clone();
    assert!(sum.is_valid());
    let sum_alt = x.clone() + &y;
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = &x + y.clone();
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = &x + &y;
    assert!(sum_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let mut x_alt = x.clone();
    x_alt += y.clone();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));

    let mut x_alt = x.clone();
    x_alt += &y;
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));

    let sum_alt = add_prec_round_naive(
        x.clone(),
        y.clone(),
        max(x.significant_bits(), y.significant_bits()),
        RoundingMode::Nearest,
    )
    .0;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = x
        .add_prec_round_ref_ref(
            &y,
            max(x.significant_bits(), y.significant_bits()),
            RoundingMode::Nearest,
        )
        .0;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = x
        .add_prec_ref_ref(&y, max(x.significant_bits(), y.significant_bits()))
        .0;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    let sum_alt = x.add_round_ref_ref(&y, RoundingMode::Nearest).0;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    if sum.is_finite() && x.is_normal() && y.is_normal() && sum.is_normal() {
        assert_eq!(
            sum.get_prec(),
            Some(max(x.get_prec().unwrap(), y.get_prec().unwrap()))
        );
        let r_sum = Rational::exact_from(&x) + Rational::exact_from(&y);
        if sum < r_sum {
            let mut next = sum.clone();
            next.increment();
            assert!(next > r_sum);
        } else if sum > r_sum {
            let mut next = sum.clone();
            next.decrement();
            assert!(next < r_sum);
        }
    }

    let rug_sum = rug_add(rug::Float::exact_from(&x), rug::Float::exact_from(&y));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_sum)),
        ComparableFloatRef(&sum),
    );

    let sum_alt = &y + &x;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    let sum_alt = &x - -&y;
    assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

    if (x != 0u32 && y != 0u32) || (x.is_sign_positive() && y.is_sign_positive()) {
        let sum_alt = (-(-&x - &y)).abs_negative_zero();
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

        let sum_alt = (-(-&x + -&y)).abs_negative_zero();
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
    }

    // example of associativity failure: 0x1.0#1 0x2.0#1 -0x1.0#1
}

#[test]
fn add_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        add_properties_helper_2(x, y);
    });

    float_pair_gen_var_2().test_properties(|(x, y)| {
        add_properties_helper_2(x, y);
    });

    float_pair_gen_var_3().test_properties(|(x, y)| {
        add_properties_helper_2(x, y);
    });

    float_pair_gen_var_4().test_properties(|(x, y)| {
        add_properties_helper_2(x, y);
    });

    float_pair_gen_var_5().test_properties(|(x, y)| {
        add_properties_helper_2(x, y);
    });

    float_pair_gen_var_6().test_properties(|(x, y)| {
        add_properties_helper_2(x, y);
    });

    float_pair_gen_var_7().test_properties(|(x, y)| {
        add_properties_helper_2(x, y);
    });

    apply_fn_to_primitive_floats!(add_properties_helper_1);

    float_gen().test_properties(|x| {
        assert!((&x + Float::NAN).is_nan());
        assert!((Float::NAN + &x).is_nan());
        if !x.is_nan() {
            if x != Float::NEGATIVE_INFINITY {
                assert_eq!(&x + Float::INFINITY, Float::INFINITY);
                assert_eq!(Float::INFINITY + &x, Float::INFINITY);
            }
            if x != Float::INFINITY {
                assert_eq!(&x + Float::NEGATIVE_INFINITY, Float::NEGATIVE_INFINITY);
                assert_eq!(Float::NEGATIVE_INFINITY + &x, Float::NEGATIVE_INFINITY);
            }
        }
        if !x.is_negative_zero() {
            assert_eq!(
                ComparableFloatRef(&(&x + Float::ZERO)),
                ComparableFloatRef(&x)
            );
            assert_eq!(
                ComparableFloatRef(&(Float::ZERO + &x)),
                ComparableFloatRef(&x)
            );
        }
        assert_eq!(
            ComparableFloatRef(&(&x + Float::NEGATIVE_ZERO)),
            ComparableFloatRef(&x)
        );
        assert_eq!(
            ComparableFloatRef(&(Float::NEGATIVE_ZERO + &x)),
            ComparableFloatRef(&x)
        );
    });
}

#[test]
fn add_rational_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_1().test_properties(
        |(x, y, prec, rm)| {
            let (sum, o) = x.clone().add_rational_prec_round(y.clone(), prec, rm);
            assert!(sum.is_valid());
            let (sum_alt, o_alt) = x.clone().add_rational_prec_round_val_ref(&y, prec, rm);
            assert!(sum_alt.is_valid());
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);
            let (sum_alt, o_alt) = x.add_rational_prec_round_ref_val(y.clone(), prec, rm);
            assert!(sum_alt.is_valid());
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);
            let (sum_alt, o_alt) = x.add_rational_prec_round_ref_ref(&y, prec, rm);
            assert!(sum_alt.is_valid());
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);

            let mut x_alt = x.clone();
            let o_alt = x_alt.add_rational_prec_round_assign(y.clone(), prec, rm);
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);

            let mut x_alt = x.clone();
            let o_alt = x_alt.add_rational_prec_round_assign_ref(&y, prec, rm);
            assert!(x_alt.is_valid());
            assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);

            let (sum_alt, o_alt) = add_rational_prec_round_naive(x.clone(), y.clone(), prec, rm);
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);

            let r_sum = if sum.is_finite() {
                if sum.is_normal() {
                    assert_eq!(sum.get_prec(), Some(prec));
                }
                let r_sum = Rational::exact_from(&x) + &y;
                assert_eq!(sum.partial_cmp(&r_sum), Some(o));
                if o == Ordering::Less {
                    let mut next = sum.clone();
                    next.increment();
                    assert!(next > r_sum);
                } else if o == Ordering::Greater {
                    let mut next = sum.clone();
                    next.decrement();
                    assert!(next < r_sum);
                }
                Some(r_sum)
            } else {
                assert_eq!(o, Ordering::Equal);
                None
            };

            match (r_sum.is_some() && *r_sum.as_ref().unwrap() >= 0u32, rm) {
                (_, RoundingMode::Floor)
                | (true, RoundingMode::Down)
                | (false, RoundingMode::Up) => {
                    assert_ne!(o, Ordering::Greater)
                }
                (_, RoundingMode::Ceiling)
                | (true, RoundingMode::Up)
                | (false, RoundingMode::Down) => {
                    assert_ne!(o, Ordering::Less)
                }
                (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
                _ => {}
            }

            let (sum_alt, o_alt) = x.sub_rational_prec_round_ref_val(-&y, prec, rm);
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);

            let (mut sum_alt, mut o_alt) = (-&x).sub_rational_prec_round_val_ref(&y, prec, -rm);
            sum_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloat(sum_alt.abs_negative_zero()),
                ComparableFloat(sum.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, o);

            let (mut sum_alt, mut o_alt) = (-x).add_rational_prec_round(-y, prec, -rm);
            sum_alt.neg_assign();
            o_alt = o_alt.reverse();
            assert_eq!(
                ComparableFloat(sum_alt.abs_negative_zero()),
                ComparableFloat(sum.abs_negative_zero())
            );
            assert_eq!(o_alt, o);
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        if !x.is_negative_zero() {
            let (sum, o) = x.add_rational_prec_round_ref_val(Rational::ZERO, prec, rm);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec_round(prec, rm);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);
        }
    });

    rational_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (sum, o) = Float::NAN.add_rational_prec_round_val_ref(&x, prec, rm);
        assert!(sum.is_nan());
        assert_eq!(o, Ordering::Equal);

        assert_eq!(
            Float::INFINITY.add_rational_prec_round_val_ref(&x, prec, rm),
            (Float::INFINITY, Ordering::Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.add_rational_prec_round_val_ref(&x, prec, rm),
            (Float::NEGATIVE_INFINITY, Ordering::Equal)
        );

        let (sum, o) = Float::ZERO.add_rational_prec_round_val_ref(&x, prec, rm);
        let (sum_alt, o_alt) = Float::from_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);

        let (sum, o) = Float::NEGATIVE_ZERO.add_rational_prec_round_val_ref(&x, prec, rm);
        let (mut sum_alt, o_alt) = Float::from_rational_prec_round_ref(&x, prec, rm);
        if x == 0u32 {
            sum_alt.neg_assign();
        }
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);
    });
}

#[test]
fn add_rational_prec_properties() {
    float_rational_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        let (sum, o) = x.clone().add_rational_prec(y.clone(), prec);
        assert!(sum.is_valid());
        let (sum_alt, o_alt) = x.clone().add_rational_prec_val_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
        let (sum_alt, o_alt) = x.add_rational_prec_ref_val(y.clone(), prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
        let (sum_alt, o_alt) = x.add_rational_prec_ref_ref(&y, prec);
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.add_rational_prec_assign(y.clone(), prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.add_rational_prec_assign_ref(&y, prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) =
            add_rational_prec_round_naive(x.clone(), y.clone(), prec, RoundingMode::Nearest);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) = x.add_rational_prec_round_ref_ref(&y, prec, RoundingMode::Nearest);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        if sum.is_finite() {
            if sum.is_normal() {
                assert_eq!(sum.get_prec(), Some(prec));
            }
            let r_sum = Rational::exact_from(&x) + &y;
            assert_eq!(sum.partial_cmp(&r_sum), Some(o));
            if o == Ordering::Less {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if o == Ordering::Greater {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }
        } else {
            assert_eq!(o, Ordering::Equal);
        }

        let (sum_alt, o_alt) = x.sub_rational_prec_ref_val(-&y, prec);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        if (x != 0u32 && y != 0u32) || x.is_sign_positive() {
            let (mut sum_alt, mut o_alt) = (-&x).sub_rational_prec_val_ref(&y, prec);
            sum_alt.neg_assign();
            sum_alt.abs_negative_zero_assign();
            o_alt = o_alt.reverse();
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);

            let (mut sum_alt, mut o_alt) = (-x).add_rational_prec(-y, prec);
            sum_alt.neg_assign();
            sum_alt.abs_negative_zero_assign();
            o_alt = o_alt.reverse();
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
            assert_eq!(o_alt, o);
        }
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        if !x.is_negative_zero() {
            let (sum, o) = x.add_rational_prec_ref_val(Rational::ZERO, prec);
            let mut sum_alt = x.clone();
            let o_alt = sum_alt.set_prec(prec);
            assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
            assert_eq!(o, o_alt);
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (sum, o) = Float::NAN.add_rational_prec_val_ref(&x, prec);
        assert!(sum.is_nan());
        assert_eq!(o, Ordering::Equal);

        assert_eq!(
            Float::INFINITY.add_rational_prec_val_ref(&x, prec),
            (Float::INFINITY, Ordering::Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.add_rational_prec_val_ref(&x, prec),
            (Float::NEGATIVE_INFINITY, Ordering::Equal)
        );
        let (sum, o) = Float::ZERO.add_rational_prec_val_ref(&x, prec);
        let (sum_alt, o_alt) = Float::from_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);

        let (sum, o) = Float::NEGATIVE_ZERO.add_rational_prec_val_ref(&x, prec);
        let (mut sum_alt, o_alt) = Float::from_rational_prec_ref(&x, prec);
        if x == 0u32 {
            sum_alt.neg_assign();
        }
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);
    });
}

#[test]
fn add_rational_round_properties() {
    float_rational_rounding_mode_triple_gen_var_1().test_properties(|(x, y, rm)| {
        let (sum, o) = x.clone().add_rational_round(y.clone(), rm);
        assert!(sum.is_valid());
        let (sum_alt, o_alt) = x.clone().add_rational_round_val_ref(&y, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let (sum_alt, o_alt) = x.add_rational_round_ref_val(y.clone(), rm);
        assert!(sum_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let (sum_alt, o_alt) = x.add_rational_round_ref_ref(&y, rm);
        assert!(sum_alt.is_valid());
        assert_eq!(o_alt, o);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

        let mut x_alt = x.clone();
        let o_alt = x_alt.add_rational_round_assign(y.clone(), rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.add_rational_round_assign_ref(&y, rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let (sum_alt, o_alt) =
            add_rational_prec_round_naive(x.clone(), y.clone(), x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);
        let (sum_alt, o_alt) = x.add_rational_prec_round_ref_ref(&y, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        assert_eq!(o_alt, o);

        let r_sum = if sum.is_finite() {
            if x.is_normal() && sum.is_normal() {
                assert_eq!(sum.get_prec(), Some(x.get_prec().unwrap()));
            }
            let r_sum = Rational::exact_from(&x) + &y;
            assert_eq!(sum.partial_cmp(&r_sum), Some(o));
            if o == Ordering::Less {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if o == Ordering::Greater {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }
            Some(r_sum)
        } else {
            assert_eq!(o, Ordering::Equal);
            None
        };

        match (r_sum.is_some() && *r_sum.as_ref().unwrap() >= 0u32, rm) {
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sum, rug_o) = rug_add_rational_round(
                rug::Float::exact_from(&x),
                rug::Rational::exact_from(&y),
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sum)),
                ComparableFloatRef(&sum)
            );
            assert_eq!(rug_o, o);
        }

        let (sum_alt, o_alt) = x.sub_rational_round_ref_val(-&y, rm);
        assert_eq!(o_alt, o);
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

        let (mut sum_alt, mut o_alt) = (-&x).sub_rational_round_val_ref(&y, -rm);
        sum_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloat(sum_alt.abs_negative_zero()),
            ComparableFloat(sum.abs_negative_zero_ref())
        );

        let (mut sum_alt, mut o_alt) = (-x).add_rational_round(-y, -rm);
        sum_alt.neg_assign();
        o_alt = o_alt.reverse();
        assert_eq!(o_alt, o);
        assert_eq!(
            ComparableFloat(sum_alt.abs_negative_zero()),
            ComparableFloat(sum.abs_negative_zero())
        );
    });

    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        if !x.is_negative_zero() {
            let (sum, o) = x.add_rational_round_ref_val(Rational::ZERO, rm);
            assert_eq!(ComparableFloat(sum), ComparableFloat(x));
            assert_eq!(o, Ordering::Equal);
        }
    });

    rational_rounding_mode_pair_gen_var_6().test_properties(|(x, rm)| {
        let (sum, o) = Float::NAN.add_rational_round_val_ref(&x, rm);
        assert!(sum.is_nan());
        assert_eq!(o, Ordering::Equal);

        assert_eq!(
            Float::INFINITY.add_rational_round_val_ref(&x, rm),
            (Float::INFINITY, Ordering::Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.add_rational_round_val_ref(&x, rm),
            (Float::NEGATIVE_INFINITY, Ordering::Equal)
        );
        let (sum, o) = Float::ZERO.add_rational_round_val_ref(&x, rm);
        let (sum_alt, o_alt) = Float::from_rational_prec_round_ref(&x, 1, rm);
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);

        let (sum, o) = Float::NEGATIVE_ZERO.add_rational_round_val_ref(&x, rm);
        let (mut sum_alt, o_alt) = Float::from_rational_prec_round_ref(&x, 1, rm);
        if x == 0u32 {
            sum_alt.neg_assign();
        }
        assert_eq!(ComparableFloat(sum), ComparableFloat(sum_alt));
        assert_eq!(o, o_alt);
    });
}

#[test]
fn add_rational_propertieszzz() {
    float_rational_pair_gen().test_properties(|(x, y)| {
        let sum = x.clone() + y.clone();
        assert!(sum.is_valid());
        let sum_alt = x.clone() + &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = &x + y.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = &x + &y;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

        let sum_alt = y.clone() + x.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = y.clone() + &x;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = &y + x.clone();
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = &y + &x;
        assert!(sum_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

        let mut x_alt = x.clone();
        x_alt += y.clone();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));

        let mut x_alt = x.clone();
        x_alt += &y;
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sum));

        let sum_alt = add_rational_prec_round_naive(
            x.clone(),
            y.clone(),
            x.significant_bits(),
            RoundingMode::Nearest,
        )
        .0;
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = x
            .add_rational_prec_round_ref_ref(&y, x.significant_bits(), RoundingMode::Nearest)
            .0;
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = x.add_rational_prec_ref_ref(&y, x.significant_bits()).0;
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        let sum_alt = x.add_rational_round_ref_ref(&y, RoundingMode::Nearest).0;
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

        if sum.is_finite() && x.is_normal() && sum.is_normal() {
            assert_eq!(sum.get_prec(), Some(x.get_prec().unwrap()));
            let r_sum = Rational::exact_from(&x) + &y;
            if sum < r_sum {
                let mut next = sum.clone();
                next.increment();
                assert!(next > r_sum);
            } else if sum > r_sum {
                let mut next = sum.clone();
                next.decrement();
                assert!(next < r_sum);
            }
        }

        let rug_sum = rug_add_rational(rug::Float::exact_from(&x), rug::Rational::from(&y));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sum)),
            ComparableFloatRef(&sum),
        );

        let sum_alt = &x - -&y;
        assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

        if (x != 0u32 && y != 0u32) || x.is_sign_positive() {
            let sum_alt = (-(-&x - &y)).abs_negative_zero();
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));

            let sum_alt = (-(-&x + -&y)).abs_negative_zero();
            assert_eq!(ComparableFloatRef(&sum_alt), ComparableFloatRef(&sum));
        }
    });

    float_gen().test_properties(|x| {
        assert_eq!(
            ComparableFloatRef(&(&x + Rational::ZERO)),
            ComparableFloatRef(&x)
        );
        assert_eq!(
            ComparableFloatRef(&(Rational::ZERO + &x)),
            ComparableFloatRef(&x)
        );
    });

    rational_gen().test_properties(|x| {
        assert!((&x + Float::NAN).is_nan());
        assert!((Float::NAN + &x).is_nan());
        assert_eq!(&x + Float::INFINITY, Float::INFINITY);
        assert_eq!(Float::INFINITY + &x, Float::INFINITY);
        assert_eq!(&x + Float::NEGATIVE_INFINITY, Float::NEGATIVE_INFINITY);
        assert_eq!(Float::NEGATIVE_INFINITY + &x, Float::NEGATIVE_INFINITY);
        let sum_alt = Float::from_rational_prec_ref(&x, 1).0;
        assert_eq!(
            ComparableFloat(&x + Float::ZERO),
            ComparableFloat(sum_alt.clone())
        );
        assert_eq!(
            ComparableFloat(Float::ZERO + &x),
            ComparableFloat(sum_alt.clone())
        );
        assert_eq!(
            ComparableFloat((&x + Float::NEGATIVE_ZERO).abs_negative_zero()),
            ComparableFloat(sum_alt.abs_negative_zero_ref())
        );
        assert_eq!(
            ComparableFloat((Float::NEGATIVE_ZERO + &x).abs_negative_zero()),
            ComparableFloat(sum_alt.abs_negative_zero())
        );
    });
}
