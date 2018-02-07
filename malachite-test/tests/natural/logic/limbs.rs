use common::test_properties;
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;
use std::u32;

#[test]
fn test_to_limbs_le() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().to_limbs_le(), out);
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("1000000000000", vec![3_567_587_328, 232]);
    test(
        "1701411834921604967429270619762735448065",
        vec![1, 2, 3, 4, 5],
    );
    test("4294967295", vec![u32::MAX]);
    test("4294967296", vec![0, 1]);
    test("18446744073709551615", vec![u32::MAX, u32::MAX]);
    test("18446744073709551616", vec![0, 0, 1]);
}

#[test]
fn test_to_limbs_be() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().to_limbs_be(), out);
    };
    test("0", vec![]);
    test("123", vec![123]);
    test("1000000000000", vec![232, 3_567_587_328]);
    test(
        "1701411834921604967429270619762735448065",
        vec![5, 4, 3, 2, 1],
    );
    test("4294967295", vec![u32::MAX]);
    test("4294967296", vec![1, 0]);
    test("18446744073709551615", vec![u32::MAX, u32::MAX]);
    test("18446744073709551616", vec![1, 0, 0]);
}

#[test]
fn to_limbs_le_properties() {
    test_properties(naturals, |x| {
        let limbs = x.to_limbs_le();
        assert_eq!(Natural::from_limbs_le(&limbs), *x);
        assert_eq!(
            x.to_limbs_be(),
            limbs.iter().cloned().rev().collect::<Vec<u32>>()
        );
        if *x != 0 {
            assert_ne!(*limbs.last().unwrap(), 0);
        }
    });
}

#[test]
fn to_limbs_be_properties() {
    test_properties(naturals, |x| {
        let limbs = x.to_limbs_be();
        assert_eq!(Natural::from_limbs_be(&limbs), *x);
        assert_eq!(
            x.to_limbs_le(),
            limbs.iter().cloned().rev().collect::<Vec<u32>>()
        );
        if *x != 0 {
            assert_ne!(limbs[0], 0);
        }
    });
}
