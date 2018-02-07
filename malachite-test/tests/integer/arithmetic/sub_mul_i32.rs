use common::test_properties;
use malachite_base::num::{NegativeOne, One, SubMul, SubMulAssign, Zero};
use malachite_nz::integer::Integer;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_signed, pairs_of_integers,
                                      triples_of_integer_integer_and_signed};

use std::str::FromStr;

#[test]
fn test_sub_mul_i32() {
    let test = |u, v, c: i32, out| {
        let mut a = Integer::from_str(u).unwrap();
        a.sub_mul_assign(Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(u).unwrap();
        a.sub_mul_assign(&Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u)
            .unwrap()
            .sub_mul(Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(u)
            .unwrap()
            .sub_mul(&Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Integer::from_str(u).unwrap()).sub_mul(Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Integer::from_str(u).unwrap()).sub_mul(&Integer::from_str(v).unwrap(), c);
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", -123, "0");
    test("123", "0", -5, "123");
    test("-123", "0", -5, "-123");
    test("123", "5", -1, "128");
    test("-123", "5", -1, "-118");
    test("123", "5", -100, "623");
    test("-123", "5", -100, "377");
    test("10", "3", -4, "22");
    test("10", "-3", -4, "-2");
    test("1000000000000", "0", -123, "1000000000000");
    test("1000000000000", "1", -123, "1000000000123");
    test("1000000000000", "123", -1, "1000000000123");
    test("1000000000000", "123", -100, "1000000012300");
    test("1000000000000", "100", -123, "1000000012300");
    test("1000000000000", "65536", -0x1_0000, "1004294967296");
    test("-1000000000000", "-65536", -0x1_0000, "-1004294967296");
    test("-1000000000000", "65536", -0x1_0000, "-995705032704");
    test("1000000000000", "-65536", -0x1_0000, "995705032704");
    test("1000000000000", "1000000000000", 0, "1000000000000");
    test("1000000000000", "1000000000000", -1, "2000000000000");
    test("1000000000000", "1000000000000", -100, "101000000000000");
    test("0", "1000000000000", -100, "100000000000000");
    test("-1", "1000000000000", -100, "99999999999999");
    test("0", "-1000000000000", -100, "-100000000000000");
    test("1", "-1000000000000", -100, "-99999999999999");
    test("0", "0", 123, "0");
    test("-123", "0", 5, "-123");
    test("123", "0", 5, "123");
    test("-123", "5", 1, "-128");
    test("123", "5", 1, "118");
    test("-123", "5", 100, "-623");
    test("123", "5", 100, "-377");
    test("-10", "3", 4, "-22");
    test("-10", "-3", 4, "2");
    test("-1000000000000", "0", 123, "-1000000000000");
    test("-1000000000000", "1", 123, "-1000000000123");
    test("-1000000000000", "123", 1, "-1000000000123");
    test("-1000000000000", "123", 100, "-1000000012300");
    test("-1000000000000", "100", 123, "-1000000012300");
    test("-1000000000000", "65536", 0x1_0000, "-1004294967296");
    test("1000000000000", "-65536", 0x1_0000, "1004294967296");
    test("1000000000000", "65536", 0x1_0000, "995705032704");
    test("-1000000000000", "-65536", 0x1_0000, "-995705032704");
    test("-1000000000000", "1000000000000", 1, "-2000000000000");
    test("-1000000000000", "1000000000000", 100, "-101000000000000");
    test("0", "1000000000000", 100, "-100000000000000");
    test("1", "1000000000000", 100, "-99999999999999");
    test("0", "-1000000000000", 100, "100000000000000");
    test("-1", "-1000000000000", 100, "99999999999999");
    test("1000000000000", "1000000000000", 1, "0");
    test("1000000000000", "-1000000000000", -1, "0");
    test("-1000000000000", "-1000000000000", 1, "0");
    test("1000000000000", "-1000000000000", -1, "0");
    test(
        "1000000000000000000000",
        "1000000000000",
        1_000_000_000,
        "0",
    );
    test(
        "1000000000000000000000",
        "-1000000000000",
        -1_000_000_000,
        "0",
    );
    test(
        "-1000000000000000000000",
        "-1000000000000",
        1_000_000_000,
        "0",
    );
    test(
        "1000000000000000000000",
        "-1000000000000",
        -1_000_000_000,
        "0",
    );
}

#[test]
fn sub_mul_i32_properties() {
    test_properties(
        triples_of_integer_integer_and_signed,
        |&(ref a, ref b, c): &(Integer, Integer, i32)| {
            let mut mut_a = a.clone();
            mut_a.sub_mul_assign(b.clone(), c);
            assert!(mut_a.is_valid());
            let result = mut_a;

            let mut mut_a = a.clone();
            mut_a.sub_mul_assign(b, c);
            assert!(mut_a.is_valid());
            assert_eq!(mut_a, result);

            let result_alt = a.sub_mul(b.clone(), c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.sub_mul(b, c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.clone().sub_mul(b.clone(), c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = a.clone().sub_mul(b, c);
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            assert_eq!(a - b * c, result);
            assert_eq!(a.sub_mul(-b, -c), result);
            assert_eq!((-a).sub_mul(-b, c), -&result);
            assert_eq!((-a).sub_mul(b, -c), -&result);
            assert_eq!(a.sub_mul(b, &Integer::from(c)), result);
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n.sub_mul(n, 1), 0);
        assert_eq!(n.sub_mul(-n, -1), 0);
    });

    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, c): &(Integer, i32)| {
            assert_eq!(n.sub_mul(&Integer::ZERO, c), *n);
            assert_eq!(n.sub_mul(&Integer::ONE, c), n - c);
            assert_eq!(n.sub_mul(&Integer::NEGATIVE_ONE, c), n + c);
            assert_eq!(Integer::ZERO.sub_mul(n, c), -n * c);
            assert_eq!((n * c).sub_mul(n, c), 0);
            assert_eq!((n * c).sub_mul(-n, -c), 0);
        },
    );

    test_properties(pairs_of_integers, |&(ref a, ref b)| {
        assert_eq!(a.sub_mul(b, 0), *a);
        assert_eq!(a.sub_mul(b, 1), a - b);
        assert_eq!(a.sub_mul(b, -1), a + b);
    });
}
