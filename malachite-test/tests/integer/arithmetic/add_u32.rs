use common::LARGE_LIMIT;
use malachite_base::num::Zero;
use malachite_nz::integer::Integer;
use malachite_test::common::{bigint_to_integer, integer_to_bigint, integer_to_rug_integer,
                             rug_integer_to_integer, GenerationMode};
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned};
use malachite_test::integer::arithmetic::add_u32::num_add_u32;
use num::BigInt;
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_add_u32() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n += v;
        assert_eq!(n.to_string(), out);

        let n = Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_add_u32(BigInt::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() + v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = v + rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v + &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from(0);
        n.assign(v + &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };
    test("0", 0, "0");
    test("0", 123, "123");
    test("123", 0, "123");
    test("123", 456, "579");
    test("-123", 456, "333");
    test("-500", 456, "-44");
    test("1000000000000", 123, "1000000000123");
    test("-1000000000000", 123, "-999999999877");
    test("4294967295", 1, "4294967296");
    test("-4294967296", 1, "-4294967295");
    test("2147483647", 1, "2147483648");
    test("-2147483648", 1, "-2147483647");
    test("18446744073709551615", 1, "18446744073709551616");
    test("-18446744073709551616", 1, "-18446744073709551615");
}

#[test]
fn add_u32_properties() {
    // n += u is equivalent for malachite and rug.
    // n + u is equivalent for malachite, num, and rug.
    // &n + u is equivalent for malachite and num.
    // n += u; n is valid.
    // n + u and u + n are valid.
    // &n + u and u + &n are valid.
    // n += u, n + u, u + n, &n + u, and u + &n give the same result.
    // n + u == n + from(u)
    // n + u - u == n
    // n + u - n == u
    let integer_and_u32 = |mut n: Integer, u: u32| {
        let old_n = n.clone();
        n += u;
        assert!(n.is_valid());

        let mut rug_n = integer_to_rug_integer(&old_n);
        rug_n += u;
        assert_eq!(rug_integer_to_integer(&rug_n), n);

        let n2 = old_n.clone();
        let result = &n2 + u;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = n2 + u;
        assert!(result.is_valid());
        assert_eq!(result, n);

        let n2 = old_n.clone();
        let result = u + &n2;
        assert!(result.is_valid());
        assert_eq!(result, n);
        let result = u + n2;
        assert_eq!(result, n);
        assert!(result.is_valid());

        let n2 = old_n.clone();
        let result = n2 + Integer::from(u);
        assert_eq!(result, n);
        let n2 = old_n.clone();
        let result = Integer::from(u) + n2;
        assert_eq!(result, n);

        let num_n2 = integer_to_bigint(&old_n);
        assert_eq!(bigint_to_integer(&num_add_u32(num_n2, u)), n);

        let rug_n2 = integer_to_rug_integer(&old_n);
        assert_eq!(rug_integer_to_integer(&(rug_n2 + u)), n);

        assert_eq!(&n - u, old_n);
        assert_eq!(n - old_n, u);
    };

    // n + 0 == n
    // 0 + n == n
    #[allow(unknown_lints, identity_op)]
    let one_integer = |n: Integer| {
        assert_eq!(&n + 0, n);
        assert_eq!(0 + &n, n);
    };

    // 0 + u == u
    // u + 0 == u
    let one_u32 = |u: u32| {
        assert_eq!(Integer::ZERO + u, u);
        assert_eq!(u + Integer::ZERO, u);
    };

    for (n, u) in pairs_of_integer_and_unsigned(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for (n, u) in pairs_of_integer_and_unsigned(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u32(n, u);
    }

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in unsigneds(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_u32(n);
    }

    for n in unsigneds(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_u32(n);
    }
}
