use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode;

use generators::common::Generator;
use generators::exhaustive::{
    exhaustive_bool_gen, exhaustive_char_gen, exhaustive_char_gen_var_1, exhaustive_char_gen_var_2,
    exhaustive_char_gen_var_3, exhaustive_primitive_int_gen_var_1, exhaustive_rounding_mode_gen,
    exhaustive_signed_gen, exhaustive_signed_gen_var_1, exhaustive_signed_gen_var_2,
    exhaustive_unsigned_gen,
};
use generators::random::{
    random_bool_gen, random_char_gen, random_char_gen_var_1, random_char_gen_var_2,
    random_char_gen_var_3, random_primitive_int_gen, random_rounding_mode_gen,
    random_signed_gen_var_1, random_signed_gen_var_2, random_unsigned_gen_var_1,
};
use generators::special_random::{
    special_random_char_gen, special_random_char_gen_var_1, special_random_char_gen_var_2,
    special_random_char_gen_var_3, special_random_signed_gen, special_random_signed_gen_var_1,
    special_random_signed_gen_var_2, special_random_unsigned_gen,
    special_random_unsigned_gen_var_1,
};

// -- bool --

pub fn bool_gen() -> Generator<bool> {
    Generator::new_no_special(&exhaustive_bool_gen, &random_bool_gen)
}

// -- char --

pub fn char_gen() -> Generator<char> {
    Generator::new(
        &exhaustive_char_gen,
        &random_char_gen,
        &special_random_char_gen,
    )
}

/// All ASCII `char`s.
pub fn char_gen_var_1() -> Generator<char> {
    Generator::new(
        &exhaustive_char_gen_var_1,
        &random_char_gen_var_1,
        &special_random_char_gen_var_1,
    )
}

/// All `char`s except for `char::MAX`.
pub fn char_gen_var_2() -> Generator<char> {
    Generator::new(
        &exhaustive_char_gen_var_2,
        &random_char_gen_var_2,
        &special_random_char_gen_var_2,
    )
}

/// All `char`s except for `char::MIN`.
pub fn char_gen_var_3() -> Generator<char> {
    Generator::new(
        &exhaustive_char_gen_var_3,
        &random_char_gen_var_3,
        &special_random_char_gen_var_3,
    )
}

// -- PrimitiveSigned --

pub fn signed_gen<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen,
        &random_primitive_int_gen,
        &special_random_signed_gen,
    )
}

/// All `T`s where `T` is signed and the `T` is not `T::MIN`.
pub fn signed_gen_var_1<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_1,
        &random_signed_gen_var_1,
        &special_random_signed_gen_var_1,
    )
}

/// All signed natural (non-negative) `T`s.
pub fn signed_gen_var_2<T: PrimitiveSigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_signed_gen_var_2,
        &random_signed_gen_var_2,
        &special_random_signed_gen_var_2,
    )
}

// -- PrimitiveUnsigned --

pub fn unsigned_gen<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_unsigned_gen,
        &random_primitive_int_gen,
        &special_random_unsigned_gen,
    )
}

/// All `T` where `T` is unsigned and the `T` is positive.
pub fn unsigned_gen_var_1<T: PrimitiveUnsigned>() -> Generator<T> {
    Generator::new(
        &exhaustive_primitive_int_gen_var_1,
        &random_unsigned_gen_var_1,
        &special_random_unsigned_gen_var_1,
    )
}

// -- RoundingMode --

pub fn rounding_mode_gen() -> Generator<RoundingMode> {
    Generator::new_no_special(&exhaustive_rounding_mode_gen, &random_rounding_mode_gen)
}

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
