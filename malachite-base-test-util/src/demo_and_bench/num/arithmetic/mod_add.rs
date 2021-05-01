use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::triple_3_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::unsigned_triple_gen_var_12;
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_mod_add);
    register_unsigned_demos!(runner, demo_mod_add_assign);
    register_unsigned_benches!(runner, benchmark_mod_add);
    register_unsigned_benches!(runner, benchmark_mod_add_assign);
}

fn demo_mod_add<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y, m) in unsigned_triple_gen_var_12::<T>()
        .get(gm, &config)
        .take(limit)
    {
        println!("{} + {} === {} mod {}", x, y, x.mod_add(y, m), m);
    }
}

fn demo_mod_add_assign<T: PrimitiveUnsigned>(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, y, m) in unsigned_triple_gen_var_12::<T>()
        .get(gm, &config)
        .take(limit)
    {
        let old_x = x;
        x.mod_add_assign(y, m);
        println!("x := {}; x.mod_add_assign({}, {}); x = {}", old_x, y, m, x);
    }
}

fn benchmark_mod_add<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_add({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_12::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [("Malachite", &mut |(x, y, m)| no_out!(x.mod_add(y, m)))],
    );
}

fn benchmark_mod_add_assign<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.mod_add({}, {})", T::NAME, T::NAME, T::NAME),
        BenchmarkType::Single,
        unsigned_triple_gen_var_12::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_bit_bucketer("m"),
        &mut [("Malachite", &mut |(mut x, y, m)| x.mod_add_assign(y, m))],
    );
}
