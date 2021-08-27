use crate::bench::bucketers::{pair_1_integer_bit_bucketer, triple_3_pair_1_integer_bit_bucketer};
use malachite_base::num::arithmetic::traits::{
    CeilingRoot, CeilingRootAssign, CheckedRoot, FloorRoot, FloorRootAssign,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::generators::{
    integer_unsigned_pair_gen_var_3, integer_unsigned_pair_gen_var_3_nrm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_floor_root);
    register_demo!(runner, demo_integer_floor_root_ref);
    register_demo!(runner, demo_integer_floor_root_assign);
    register_demo!(runner, demo_integer_ceiling_root);
    register_demo!(runner, demo_integer_ceiling_root_ref);
    register_demo!(runner, demo_integer_ceiling_root_assign);
    register_demo!(runner, demo_integer_checked_root);
    register_demo!(runner, demo_integer_checked_root_ref);
    register_bench!(runner, benchmark_integer_floor_root_evaluation_strategy);
    register_bench!(runner, benchmark_integer_floor_root_library_comparison);
    register_bench!(runner, benchmark_integer_floor_root_assign);
    register_bench!(runner, benchmark_integer_ceiling_root_evaluation_strategy);
    register_bench!(runner, benchmark_integer_ceiling_root_assign);
    register_bench!(runner, benchmark_integer_checked_root_evaluation_strategy);
}

fn demo_integer_floor_root(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in integer_unsigned_pair_gen_var_3()
        .get(gm, &config)
        .take(limit)
    {
        println!("{}.floor_root({}) = {}", x, exp, x.clone().floor_root(exp));
    }
}

fn demo_integer_floor_root_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in integer_unsigned_pair_gen_var_3()
        .get(gm, &config)
        .take(limit)
    {
        println!("(&{}).floor_root({}) = {}", x, exp, (&x).floor_root(exp));
    }
}

fn demo_integer_floor_root_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, exp) in integer_unsigned_pair_gen_var_3()
        .get(gm, &config)
        .take(limit)
    {
        let old_x = x.clone();
        x.floor_root_assign(exp);
        println!("x := {}; x.floor_root_assign(); x = {}", old_x, x);
    }
}

fn demo_integer_ceiling_root(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in integer_unsigned_pair_gen_var_3()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.ceiling_root({}) = {}",
            x,
            exp,
            x.clone().ceiling_root(exp)
        );
    }
}

fn demo_integer_ceiling_root_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in integer_unsigned_pair_gen_var_3()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "(&{}).ceiling_root({}) = {}",
            x,
            exp,
            (&x).ceiling_root(exp)
        );
    }
}

fn demo_integer_ceiling_root_assign(gm: GenMode, config: GenConfig, limit: usize) {
    for (mut x, exp) in integer_unsigned_pair_gen_var_3()
        .get(gm, &config)
        .take(limit)
    {
        let old_x = x.clone();
        x.ceiling_root_assign(exp);
        println!("x := {}; x.ceiling_root_assign({}); x = {}", old_x, exp, x);
    }
}

fn demo_integer_checked_root(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in integer_unsigned_pair_gen_var_3()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "{}.checked_root({}) = {:?}",
            x,
            exp,
            x.clone().checked_root(exp)
        );
    }
}

fn demo_integer_checked_root_ref(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, exp) in integer_unsigned_pair_gen_var_3()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "(&{}).checked_root({}) = {:?}",
            x,
            exp,
            (&x).checked_root(exp)
        );
    }
}

fn benchmark_integer_floor_root_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.floor_root(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_pair_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.floor_root(u64)", &mut |(x, exp)| {
                no_out!(x.floor_root(exp))
            }),
            ("(&Integer).floor_root(u64)", &mut |(x, exp)| {
                no_out!((&x).floor_root(exp))
            }),
        ],
    );
}

fn benchmark_integer_floor_root_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.floor_root()",
        BenchmarkType::LibraryComparison,
        integer_unsigned_pair_gen_var_3_nrm::<u64>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("num", &mut |((x, exp), _, _)| {
                no_out!(x.nth_root(u32::exact_from(exp)))
            }),
            ("rug", &mut |(_, (x, exp), _)| {
                no_out!(x.root(u32::exact_from(exp)))
            }),
            ("Malachite", &mut |(_, _, (x, exp))| {
                no_out!(x.floor_root(exp))
            }),
        ],
    );
}

fn benchmark_integer_floor_root_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.floor_root_assign(u64)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, exp)| x.floor_root_assign(exp))],
    );
}

fn benchmark_integer_ceiling_root_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_root(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_pair_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.ceiling_root(u64)", &mut |(x, exp)| {
                no_out!(x.ceiling_root(exp))
            }),
            ("(&Integer).ceiling_root(u64)", &mut |(x, exp)| {
                no_out!((&x).ceiling_root(exp))
            }),
        ],
    );
}

fn benchmark_integer_ceiling_root_assign(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.ceiling_root_assign(u64)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(mut x, exp)| x.ceiling_root_assign(exp))],
    );
}

fn benchmark_integer_checked_root_evaluation_strategy(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.checked_root(u64)",
        BenchmarkType::EvaluationStrategy,
        integer_unsigned_pair_gen_var_3().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Integer.checked_root(u64)", &mut |(x, exp)| {
                no_out!(x.checked_root(exp))
            }),
            ("(&Integer).checked_root(u64)", &mut |(x, exp)| {
                no_out!((&x).checked_root(exp))
            }),
        ],
    );
}
