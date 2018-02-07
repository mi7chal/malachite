use common::test_properties;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rug_integer, natural_to_rug_integer};
use malachite_test::inputs::integer::{pairs_of_integer_and_natural,
                                      triples_of_integer_natural_and_integer,
                                      triples_of_natural_integer_and_natural};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_ord_integer_natural() {
    let test = |u, v, out| {
        assert_eq!(
            Integer::from_str(u)
                .unwrap()
                .partial_cmp(&Natural::from_str(v).unwrap(),),
            out
        );

        assert_eq!(
            Natural::from_str(v)
                .unwrap()
                .partial_cmp(&Integer::from_str(u).unwrap())
                .map(|o| o.reverse()),
            out
        );
    };
    test("0", "0", Some(Ordering::Equal));
    test("0", "5", Some(Ordering::Less));
    test("123", "123", Some(Ordering::Equal));
    test("123", "124", Some(Ordering::Less));
    test("123", "122", Some(Ordering::Greater));
    test("1000000000000", "123", Some(Ordering::Greater));
    test("123", "1000000000000", Some(Ordering::Less));
    test("1000000000000", "1000000000000", Some(Ordering::Equal));
    test("-1000000000000", "1000000000000", Some(Ordering::Less));
    test("-1000000000000", "0", Some(Ordering::Less));
}

#[test]
fn partial_cmp_integer_natural_properties() {
    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        let cmp = x.partial_cmp(y);
        assert_eq!(x.cmp(&y.to_integer()), cmp.unwrap());
        assert_eq!(
            integer_to_rug_integer(x).partial_cmp(&natural_to_rug_integer(y)),
            cmp
        );
        assert_eq!(y.partial_cmp(x), cmp.map(|o| o.reverse()));
    });

    test_properties(
        triples_of_integer_natural_and_integer,
        |&(ref x, ref y, ref z)| {
            if x < y && y < z {
                assert!(x < z);
            } else if x > y && y > z {
                assert!(x > z);
            }
        },
    );

    test_properties(
        triples_of_natural_integer_and_natural,
        |&(ref x, ref y, ref z)| {
            if x < y && y < z {
                assert!(x < z);
            } else if x > y && y > z {
                assert!(x > z);
            }
        },
    );
}
