use common::LARGE_LIMIT;
use malachite_base::num::BitAccess;
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer, GenerationMode};
use malachite_test::inputs::integer::pairs_of_integer_and_small_u64;
use rug;
use std::str::FromStr;

#[test]
fn test_flip_bit() {
    let test = |u, index, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.flip_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n.toggle_bit(index as u32);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "1024");
    test("1024", 10, "0");
    test("100", 0, "101");
    test("101", 0, "100");
    test("1000000000000", 10, "1000000001024");
    test("1000000001024", 10, "1000000000000");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("1267650600228229402496703205376", 100, "1000000000000");
    test("5", 100, "1267650600228229401496703205381");
    test("1267650600228229401496703205381", 100, "5");
}

#[test]
fn flip_bit_properties() {
    // n.flip_bit(index) is equivalent for malachite and rug.
    // Flipping a bit once always changes a number.
    // Flipping the same bit twice leaves a number unchanged.
    let integer_and_u64 = |mut n: Integer, index: u64| {
        let old_n = n.clone();
        n.flip_bit(index);
        assert!(n.is_valid());
        assert_ne!(n, old_n);

        let mut rug_n = integer_to_rug_integer(&old_n);
        rug_n.toggle_bit(index as u32);
        assert_eq!(rug_integer_to_integer(&rug_n), n);

        n.flip_bit(index);
        assert_eq!(n, old_n);
    };

    for (n, index) in pairs_of_integer_and_small_u64(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        integer_and_u64(n, index);
    }

    for (n, index) in pairs_of_integer_and_small_u64(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        integer_and_u64(n, index);
    }
}
