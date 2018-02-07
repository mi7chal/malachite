use common::test_properties;
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;
use std::u32;

#[test]
fn test_trailing_zeros() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().trailing_zeros(), out);
    };
    test("0", None);
    test("123", Some(0));
    test("1000000000000", Some(12));
    test("4294967295", Some(0));
    test("4294967296", Some(32));
    test("18446744073709551615", Some(0));
    test("18446744073709551616", Some(64));
}

#[test]
fn trailing_zeros_properties() {
    test_properties(naturals, |x| {
        let trailing_zeros = x.trailing_zeros();
        assert_eq!(trailing_zeros.is_none(), *x == 0);
        if *x != 0 {
            let trailing_zeros = trailing_zeros.unwrap();
            if trailing_zeros <= u64::from(u32::MAX) {
                let trailing_zeros = trailing_zeros as u32;
                assert!((x >> trailing_zeros).is_odd());
                assert_eq!(x >> trailing_zeros << trailing_zeros, *x);
            }
        }
    });
}
