use std::fmt::Debug;

use malachite_base_test_util::generators::unsigned_gen;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    FromOtherTypeSlice, VecFromOtherType, VecFromOtherTypeSlice,
};

#[test]
pub fn test_vec_from_other_type() {
    fn test<T: Debug + Eq, U: Debug + Eq + VecFromOtherType<T>>(value: T, vec: &[U]) {
        assert_eq!(U::vec_from_other_type(value), vec);
    };
    test::<u32, u32>(123, &[123]);
    test::<u8, u16>(0xab, &[0xab]);
    test::<u16, u8>(0xcdab, &[0xab, 0xcd]);
}

fn vec_from_other_type_helper<
    T: PrimitiveUnsigned + FromOtherTypeSlice<U>,
    U: PrimitiveUnsigned + VecFromOtherType<T> + VecFromOtherTypeSlice<T>,
>() {
    unsigned_gen::<T>().test_properties(|x| {
        let xs = U::vec_from_other_type(x);
        assert_eq!(U::vec_from_other_type_slice(&[x]), xs);
        assert_eq!(T::from_other_type_slice(&xs), x);
    });
}

#[test]
fn vec_from_other_type_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(vec_from_other_type_helper);
}
