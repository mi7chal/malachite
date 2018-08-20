use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::pairs_of_unsigned_vec_and_small_u64_var_1;
use inputs::natural::pairs_of_natural_and_small_unsigned;
use malachite_base::num::{DivisibleByPowerOfTwo, SignificantBits};
use malachite_nz::natural::arithmetic::divisible_by_power_of_two::limbs_divisible_by_power_of_two;
use std::cmp::min;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_divisible_by_power_of_two);
    register_demo!(registry, demo_natural_divisible_by_power_of_two);
    register_bench!(registry, Small, benchmark_limbs_divisible_by_power_of_two);
    register_bench!(
        registry,
        Large,
        benchmark_natural_divisible_by_power_of_two_algorithms
    );
}

fn demo_limbs_divisible_by_power_of_two(gm: GenerationMode, limit: usize) {
    for (limbs, pow) in pairs_of_unsigned_vec_and_small_u64_var_1(gm).take(limit) {
        println!(
            "limbs_divisible_by_power_of_two({:?}, {}) = {:?}",
            limbs,
            pow,
            limbs_divisible_by_power_of_two(&limbs, pow)
        );
    }
}

fn demo_natural_divisible_by_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        if n.divisible_by_power_of_two(pow) {
            println!("{} is divisible by 2^{}", n, pow);
        } else {
            println!("{} is not divisible by 2^{}", n, pow);
        }
    }
}

fn benchmark_limbs_divisible_by_power_of_two(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_shr_exact(&[u32], u32)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_and_small_u64_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref limbs, pow)| min(pow as usize, limbs.len())),
        "min(pow, limbs.len())",
        &mut [(
            "malachite",
            &mut (|(limbs, pow)| no_out!(limbs_divisible_by_power_of_two(&limbs, pow))),
        )],
    );
}

fn benchmark_natural_divisible_by_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.divisible_by_power_of_two(u64)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, pow)| min(pow, n.significant_bits()) as usize),
        "min(pow, n.significant_bits())",
        &mut [
            (
                "Natural.divisible_by_power_of_2(u64)",
                &mut (|(n, pow)| no_out!(n.divisible_by_power_of_two(pow))),
            ),
            (
                "Natural.trailing_zeros().map_or(true, |z| z >= u64)",
                &mut (|(n, pow)| no_out!(n.trailing_zeros().map_or(true, |z| z >= pow))),
            ),
        ],
    );
}
