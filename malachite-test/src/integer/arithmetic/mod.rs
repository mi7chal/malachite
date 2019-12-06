use common::DemoBenchRegistry;

pub mod abs;
pub mod add;
pub mod add_limb;
pub mod add_mul;
pub mod add_mul_limb;
pub mod add_mul_signed_limb;
pub mod add_natural;
pub mod add_signed_limb;
pub mod div_exact_limb;
pub mod div_exact_signed_limb;
pub mod div_limb;
pub mod div_mod_limb;
pub mod div_mod_signed_limb;
pub mod div_round_limb;
pub mod div_round_signed_limb;
pub mod div_signed_limb;
pub mod divisible_by_limb;
pub mod divisible_by_power_of_two;
pub mod divisible_by_signed_limb;
pub mod eq_limb_mod_limb;
pub mod eq_limb_mod_power_of_two;
pub mod eq_mod_power_of_two;
pub mod eq_natural_mod_power_of_two;
pub mod eq_signed_limb_mod_power_of_two;
pub mod eq_signed_limb_mod_signed_limb;
pub mod mod_limb;
pub mod mod_power_of_two;
pub mod mod_signed_limb;
pub mod mul;
pub mod mul_natural;
pub mod neg;
pub mod parity;
pub mod shl_i;
pub mod shl_u;
pub mod shr_i;
pub mod shr_u;
pub mod sub;
pub mod sub_limb;
pub mod sub_mul;
pub mod sub_mul_limb;
pub mod sub_mul_signed_limb;
pub mod sub_natural;
pub mod sub_signed_limb;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    abs::register(registry);
    add::register(registry);
    add_limb::register(registry);
    add_mul::register(registry);
    add_mul_limb::register(registry);
    add_mul_signed_limb::register(registry);
    add_natural::register(registry);
    add_signed_limb::register(registry);
    div_exact_limb::register(registry);
    div_exact_signed_limb::register(registry);
    div_limb::register(registry);
    div_signed_limb::register(registry);
    div_mod_limb::register(registry);
    div_mod_signed_limb::register(registry);
    div_round_limb::register(registry);
    div_round_signed_limb::register(registry);
    divisible_by_limb::register(registry);
    divisible_by_power_of_two::register(registry);
    divisible_by_signed_limb::register(registry);
    eq_limb_mod_limb::register(registry);
    eq_limb_mod_power_of_two::register(registry);
    eq_mod_power_of_two::register(registry);
    eq_natural_mod_power_of_two::register(registry);
    eq_signed_limb_mod_power_of_two::register(registry);
    eq_signed_limb_mod_signed_limb::register(registry);
    mod_limb::register(registry);
    mod_power_of_two::register(registry);
    mod_signed_limb::register(registry);
    mul::register(registry);
    mul_natural::register(registry);
    neg::register(registry);
    parity::register(registry);
    shl_i::register(registry);
    shl_u::register(registry);
    shr_i::register(registry);
    shr_u::register(registry);
    sub::register(registry);
    sub_limb::register(registry);
    sub_mul::register(registry);
    sub_mul_limb::register(registry);
    sub_mul_signed_limb::register(registry);
    sub_natural::register(registry);
    sub_signed_limb::register(registry);
}
