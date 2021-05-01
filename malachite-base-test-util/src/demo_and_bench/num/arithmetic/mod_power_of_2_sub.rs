use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::triple_3_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_triple_gen_var_11;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_power_of_2_sub);
    register_unsigned_demos!(runner, demo_mod_power_of_2_sub_assign);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_sub);
    register_unsigned_benches!(runner, benchmark_mod_power_of_2_sub_assign);
}

fn demo_mod_power_of_2_sub<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, pow) in unsigned_triple_gen_var_11::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{} - {} === {} mod 2^{}",
            x,
            y,
            x.mod_power_of_2_sub(y, pow),
            pow
        );
    }
}

fn demo_mod_power_of_2_sub_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (mut x, y, pow) in unsigned_triple_gen_var_11::<T>()
        .get(gm, &config)
        .take(limit)
    {
        let old_x = x;
        x.mod_power_of_2_sub_assign(y, pow);
        println!(
            "x := {}; x.mod_power_of_2_sub_assign({}, {}); x = {}",
            old_x, y, pow, x
        );
    }
}

fn benchmark_mod_power_of_2_sub<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_sub({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_11::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(x, y, pow)| {
            no_out!(x.mod_power_of_2_sub(y, pow))
        })],
    );
}

fn benchmark_mod_power_of_2_sub_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_power_of_2_sub_assign({}, u64)", T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_11::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bucketer("pow"),
        &mut [("Malachite", &mut |(mut x, y, pow)| {
            x.mod_power_of_2_sub_assign(y, pow)
        })],
    );
}
