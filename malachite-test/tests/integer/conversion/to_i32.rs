use common::LARGE_LIMIT;
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_rug_integer, GenerationMode};
use malachite_test::inputs::integer::integers;
use rug;
use std::i32;
use std::str::FromStr;

#[test]
fn test_to_i32() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().to_i32(), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", Some(-123));
    test("1000000000000", None);
    test("-1000000000000", None);
    test("2147483647", Some(i32::MAX));
    test("2147483648", None);
    test("-2147483648", Some(i32::MIN));
    test("-2147483649", None);
}

#[test]
fn test_to_i32_wrapping() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().to_i32_wrapping(), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("-123", -123);
    test("1000000000000", -727_379_968);
    test("-1000000000000", 727_379_968);
    test("2147483647", i32::MAX);
    test("2147483648", i32::MIN);
    test("-2147483648", i32::MIN);
    test("-2147483649", i32::MAX);
}

#[test]
fn to_i32_properties() {
    // x.to_i32() is equivalent for malachite and rug.
    // if -2^31 ≤ x < 2^31, from(x.to_i32().unwrap()) == x
    // if -2^31 ≤ x < 2^31, x.to_i32() == Some(x.to_i32_wrapping())
    // if x < -2^31 or x >= 2^31, x.to_i32().is_none()
    let one_integer = |x: Integer| {
        let result = x.to_i32();
        assert_eq!(integer_to_rug_integer(&x).to_i32(), result);
        if x >= i32::MIN && x <= i32::MAX {
            assert_eq!(Integer::from(result.unwrap()), x);
            assert_eq!(result, Some(x.to_i32_wrapping()));
        } else {
            assert!(result.is_none());
        }
    };

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}

#[test]
fn to_i32_wrapping_properties() {
    // x.to_i32_wrapping() is equivalent for malachite and rug.
    // (-x).to_i32_wrapping() = -(x.to_i32_wrapping())
    // TODO relate with BitAnd
    let one_integer = |x: Integer| {
        let result = x.to_i32_wrapping();
        assert_eq!(integer_to_rug_integer(&x).to_i32_wrapping(), result);
        assert_eq!(-result, (-&x).to_i32_wrapping());
    };

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
