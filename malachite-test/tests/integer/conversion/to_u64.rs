use common::test_properties;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_test::inputs::integer::integers;
use std::cmp::Ordering;
use std::{u32, u64};
use std::str::FromStr;

#[test]
fn test_to_u64() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().to_u64(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", None);
    test("1000000000000", Some(1_000_000_000_000));
    test("-1000000000000", None);
    test("1000000000000000000000", None);
    test("-1000000000000000000000", None);
    test("4294967295", Some(u32::MAX.into()));
    test("4294967296", Some(u64::from(u32::MAX) + 1));
    test("18446744073709551615", Some(u64::MAX));
    test("18446744073709551616", None);
}

#[test]
fn test_to_u64_wrapping() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().to_u64_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("-123", 18_446_744_073_709_551_493);
    test("1000000000000", 1_000_000_000_000);
    test("-1000000000000", 18_446_743_073_709_551_616);
    test("1000000000000000000000", 3_875_820_019_684_212_736);
    test("-1000000000000000000000", 14_570_924_054_025_338_880);
    test("4294967296", u64::from(u32::MAX) + 1);
    test("4294967297", u64::from(u32::MAX) + 2);
    test("-4294967296", 0xffff_ffff_0000_0000);
    test("-4294967295", 18_446_744_069_414_584_321);
    test("18446744073709551616", 0);
    test("18446744073709551617", 1);
    test("-18446744073709551616", 0);
    test("-18446744073709551615", 1);
}

#[test]
fn to_u64_properties() {
    test_properties(integers, |x| {
        let result = x.to_u64();
        if x.sign() != Ordering::Less && x.significant_bits() <= 64 {
            assert_eq!(Integer::from(result.unwrap()), *x);
            assert_eq!(result, Some(x.to_u64_wrapping()));
        } else {
            assert!(result.is_none());
        }
    });
}

#[test]
fn to_u64_wrapping_properties() {
    // TODO relate with BitAnd
    test_properties(integers, |x| {
        let result = x.to_u64_wrapping();
        assert_eq!(result.wrapping_add((-x).to_u64_wrapping()), 0);
    });
}
