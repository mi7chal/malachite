use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::NegativeOne;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::BitAccess;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    triples_of_signed_unsigned_width_range_and_bool_var_1,
    triples_of_unsigned_unsigned_width_range_and_bool_var_1,
};

fn assign_bit_helper_unsigned<T: PrimitiveInteger>() {
    let test = |n: u64, index, bit, out: u64| {
        let mut n = T::exact_from(n);
        n.assign_bit(index, bit);
        assert_eq!(n, T::exact_from(out));
    };

    test(100, 0, true, 101);
    test(0, 10, false, 0);
    test(0, 100, false, 0);
    test(101, 0, false, 100);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, true, 1024);
        test(1024, 10, false, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1_000_000_000_000, 10, true, 1_000_000_001_024);
        test(1_000_000_001_024, 10, false, 1_000_000_000_000);
        test(1_000_000_001_024, 100, false, 1_000_000_001_024);
    }
}

fn assign_bit_helper_signed<T: PrimitiveSigned>() {
    assign_bit_helper_unsigned::<T>();

    let test = |n: i64, index, bit, out: i64| {
        let mut n = T::exact_from(n);
        n.assign_bit(index, bit);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, true, -1);
    test(-1, 100, true, -1);
    test(-33, 5, true, -1);
    test(-32, 0, true, -31);
    test(-1, 5, false, -33);
    test(-31, 0, false, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-1_000_000_000_000, 10, true, -999_999_998_976);
        test(-1_000_000_000_000, 100, true, -1_000_000_000_000);
        test(-999_999_998_976, 10, false, -1_000_000_000_000);
    }
}

#[test]
fn test_assign_bit() {
    assign_bit_helper_unsigned::<u8>();
    assign_bit_helper_unsigned::<u16>();
    assign_bit_helper_unsigned::<u32>();
    assign_bit_helper_unsigned::<u64>();
    assign_bit_helper_signed::<i8>();
    assign_bit_helper_signed::<i16>();
    assign_bit_helper_signed::<i32>();
    assign_bit_helper_signed::<i64>();
}

macro_rules! assign_bit_fail_helper_unsigned {
    ($t:ident, $fail:ident) => {
        #[test]
        #[should_panic]
        fn $fail() {
            let mut n = $t::exact_from(5);
            n.assign_bit(100, true);
        }
    };
}

macro_rules! assign_bit_fail_helper_signed {
    ($t:ident, $fail_1:ident, $fail_2:ident) => {
        assign_bit_fail_helper_unsigned!($t, $fail_1);

        #[test]
        #[should_panic]
        fn $fail_2() {
            let mut n = $t::NEGATIVE_ONE;
            n.assign_bit(100, false);
        }
    };
}

assign_bit_fail_helper_unsigned!(u8, assign_bit_u8_fail_helper);
assign_bit_fail_helper_unsigned!(u16, assign_bit_u16_fail_helper);
assign_bit_fail_helper_unsigned!(u32, assign_bit_limb_fail_helper);
assign_bit_fail_helper_unsigned!(u64, assign_bit_u64_fail_helper);
assign_bit_fail_helper_signed!(i8, assign_bit_i8_fail_1_helper, assign_bit_i8_fail_2_helper);
assign_bit_fail_helper_signed!(
    i16,
    assign_bit_i16_fail_1_helper,
    assign_bit_i16_fail_2_helper
);
assign_bit_fail_helper_signed!(
    i32,
    assign_bit_signed_limb_fail_1_helper,
    assign_bit_signed_limb_fail_2_helper
);
assign_bit_fail_helper_signed!(
    i64,
    assign_bit_i64_fail_1_helper,
    assign_bit_i64_fail_2_helper
);

fn assign_bit_properties_helper_unsigned<T: PrimitiveUnsigned + Rand>() {
    test_properties(
        triples_of_unsigned_unsigned_width_range_and_bool_var_1,
        |&(n, index, bit)| {
            let mut mut_n: T = n;
            mut_n.assign_bit(index, bit);
        },
    );
}

fn assign_bit_properties_helper_signed<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(
        triples_of_signed_unsigned_width_range_and_bool_var_1,
        |&(n, index, bit)| {
            let mut mut_n: T = n;
            mut_n.assign_bit(index, bit);
        },
    );
}

#[test]
fn assign_bit_properties() {
    assign_bit_properties_helper_unsigned::<u8>();
    assign_bit_properties_helper_unsigned::<u16>();
    assign_bit_properties_helper_unsigned::<u32>();
    assign_bit_properties_helper_unsigned::<u64>();
    assign_bit_properties_helper_signed::<i8>();
    assign_bit_properties_helper_signed::<i16>();
    assign_bit_properties_helper_signed::<i32>();
    assign_bit_properties_helper_signed::<i64>();
}
