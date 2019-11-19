use std::cmp::{max, min};

use malachite_base::num::conversion::traits::CheckedFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::natural::arithmetic::sub::{
    limbs_slice_sub_in_place_right, limbs_sub, limbs_sub_in_place_left,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_in_place_with_overlap, limbs_sub_same_length_to_out,
    limbs_sub_same_length_to_out_with_overlap, limbs_sub_to_out, limbs_vec_sub_in_place_right,
};
use malachite_nz::platform::Limb;

use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::base::{
    pairs_of_unsigned_vec_and_small_usize_var_1, pairs_of_unsigned_vec_var_1,
    pairs_of_unsigned_vec_var_3, triples_of_unsigned_vec_unsigned_and_small_usize_var_1,
    triples_of_unsigned_vec_var_3, triples_of_unsigned_vec_var_9,
};
use inputs::natural::{
    nrm_pairs_of_naturals_var_1, pairs_of_naturals_var_1, rm_pairs_of_naturals_var_1,
};

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_limbs_sub);
    register_demo!(registry, demo_limbs_sub_same_length_to_out);
    register_demo!(registry, demo_limbs_sub_to_out);
    register_demo!(registry, demo_limbs_sub_same_length_in_place_left);
    register_demo!(registry, demo_limbs_sub_in_place_left);
    register_demo!(registry, demo_limbs_sub_same_length_in_place_right);
    register_demo!(registry, demo_limbs_slice_sub_in_place_right);
    register_demo!(registry, demo_limbs_vec_sub_in_place_right);
    register_demo!(registry, demo_limbs_sub_same_length_in_place_with_overlap);
    register_demo!(registry, demo_limbs_sub_same_length_to_out_with_overlap);
    register_demo!(registry, demo_natural_sub_assign);
    register_demo!(registry, demo_natural_sub_assign_ref);
    register_demo!(registry, demo_natural_sub);
    register_demo!(registry, demo_natural_sub_val_ref);
    register_demo!(registry, demo_natural_sub_ref_val);
    register_demo!(registry, demo_natural_sub_ref_ref);
    register_bench!(registry, Small, benchmark_limbs_sub);
    register_bench!(registry, Small, benchmark_limbs_sub_same_length_to_out);
    register_bench!(registry, Small, benchmark_limbs_sub_to_out);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_sub_same_length_in_place_left
    );
    register_bench!(registry, Small, benchmark_limbs_sub_in_place_left);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_sub_same_length_in_place_right
    );
    register_bench!(registry, Small, benchmark_limbs_slice_sub_in_place_right);
    register_bench!(registry, Small, benchmark_limbs_vec_sub_in_place_right);
    register_bench!(
        registry,
        Small,
        benchmark_limbs_sub_same_length_in_place_with_overlap_algorithms
    );
    register_bench!(
        registry,
        Small,
        benchmark_limbs_sub_same_length_to_out_with_overlap_algorithms
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_assign_library_comparison
    );
    register_bench!(
        registry,
        Large,
        benchmark_natural_sub_assign_evaluation_strategy
    );
    register_bench!(registry, Large, benchmark_natural_sub_library_comparison);
    register_bench!(registry, Large, benchmark_natural_sub_evaluation_strategy);
}

pub fn limbs_sub_same_length_in_place_with_overlap_naive(
    xs: &mut [Limb],
    right_start: usize,
) -> bool {
    let left_end = xs.len() - right_start;
    let mut x = xs[..left_end].to_vec();
    let borrow = limbs_sub_same_length_in_place_left(&mut x, &xs[right_start..]);
    xs[..left_end].copy_from_slice(&x);
    borrow
}

/// Given two slices `xs` and `ys`, computes the difference between the `Natural`s whose limbs are
/// `&xs[xs.len() - ys.len()..]` and `&ys`, and writes the limbs of the result to `&xs[..ys.len()]`.
pub fn limbs_sub_same_length_to_out_with_overlap_naive(xs: &mut [Limb], ys: &[Limb]) -> bool {
    let y_len = ys.len();
    let mut x = xs[xs.len() - y_len..].to_vec();
    let borrow = limbs_sub_same_length_in_place_left(&mut x, ys);
    xs[..y_len].copy_from_slice(&x);
    borrow
}

fn demo_limbs_sub(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_3(gm).take(limit) {
        println!("limbs_sub({:?}, {:?}) = {:?}", xs, ys, limbs_sub(&xs, &ys));
    }
}

fn demo_limbs_sub_same_length_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_3(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let borrow = limbs_sub_same_length_to_out(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_sub_same_length_to_out(&mut out, {:?}, {:?}) = \
             {}; out = {:?}",
            xs_old, ys, zs, borrow, xs
        );
    }
}

fn demo_limbs_sub_to_out(gm: GenerationMode, limit: usize) {
    for (xs, ys, zs) in triples_of_unsigned_vec_var_9(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let borrow = limbs_sub_to_out(&mut xs, &ys, &zs);
        println!(
            "out := {:?}; limbs_sub_to_out(&mut out, {:?}, {:?}) = {}; \
             out = {:?}",
            xs_old, ys, zs, borrow, xs
        );
    }
}

fn demo_limbs_sub_same_length_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let borrow = limbs_sub_same_length_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_sub_same_length_in_place_left(&mut xs, {:?}) = {}; xs = {:?}",
            xs_old, ys, borrow, xs
        );
    }
}

fn demo_limbs_sub_in_place_left(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_3(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let borrow = limbs_sub_in_place_left(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_sub_in_place_left(&mut xs, {:?}) = {}; xs = {:?}",
            xs_old, ys, borrow, xs
        );
    }
}

fn demo_limbs_sub_same_length_in_place_right(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_1(gm).take(limit) {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        let borrow = limbs_sub_same_length_in_place_right(&xs, &mut ys);
        println!(
            "ys := {:?}; limbs_sub_same_length_in_place_right({:?}, &mut ys) = {}; ys = {:?}",
            ys_old, xs, borrow, xs
        );
    }
}

fn demo_limbs_slice_sub_in_place_right(gm: GenerationMode, limit: usize) {
    for (xs, ys, len) in triples_of_unsigned_vec_unsigned_and_small_usize_var_1(gm).take(limit) {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        let borrow = limbs_slice_sub_in_place_right(&xs, &mut ys, len);
        println!(
            "ys := {:?}; limbs_slice_sub_in_place_right({:?}, &mut ys, {}) = {}; ys = {:?}",
            ys_old, xs, len, borrow, ys
        );
    }
}

fn demo_limbs_vec_sub_in_place_right(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_3(gm).take(limit) {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        let borrow = limbs_vec_sub_in_place_right(&xs, &mut ys);
        println!(
            "ys := {:?}; limbs_vec_sub_in_place_right({:?}, &mut ys) = {}; ys = {:?}",
            ys_old, xs, borrow, ys
        );
    }
}

fn demo_limbs_sub_same_length_in_place_with_overlap(gm: GenerationMode, limit: usize) {
    for (xs, right_start) in pairs_of_unsigned_vec_and_small_usize_var_1(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let borrow = limbs_sub_same_length_in_place_with_overlap(&mut xs, right_start);
        println!(
            "xs := {:?}; limbs_sub_same_length_in_place_with_overlap(&mut xs, {}) = {}; xs = {:?}",
            xs_old, right_start, borrow, xs
        );
    }
}

fn demo_limbs_sub_same_length_to_out_with_overlap(gm: GenerationMode, limit: usize) {
    for (xs, ys) in pairs_of_unsigned_vec_var_3(gm).take(limit) {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let borrow = limbs_sub_same_length_to_out_with_overlap(&mut xs, &ys);
        println!(
            "xs := {:?}; limbs_sub_same_length_to_out_with_overlap(&mut xs, {:?}) = {}; xs = {:?}",
            xs_old, ys, borrow, xs
        );
    }
}

fn demo_natural_sub_assign(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        x -= y;
        println!("x := {}; x -= {}; x = {}", x_old, y_old, x);
    }
}

fn demo_natural_sub_assign_ref(gm: GenerationMode, limit: usize) {
    for (mut x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        let x_old = x.clone();
        x -= &y;
        println!("x := {}; x -= &{}; x = {}", x_old, y, x);
    }
}

fn demo_natural_sub(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        let x_old = x.clone();
        let y_old = y.clone();
        println!("{} - {} = {}", x_old, y_old, x - y);
    }
}

fn demo_natural_sub_val_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        let x_old = x.clone();
        println!("{} - &{} = {}", x_old, y, x - &y);
    }
}

fn demo_natural_sub_ref_val(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        let y_old = y.clone();
        println!("&{} - {} = {}", x, y_old, &x - y);
    }
}

fn demo_natural_sub_ref_ref(gm: GenerationMode, limit: usize) {
    for (x, y) in pairs_of_naturals_var_1(gm).take(limit) {
        println!("&{} - &{} = {}", x, y, &x - &y);
    }
}

fn benchmark_limbs_sub(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub(&[Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [("malachite", &mut (|(xs, ys)| no_out!(limbs_sub(&xs, &ys))))],
    );
}

fn benchmark_limbs_sub_same_length_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_same_length_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ys, _)| ys.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys, zs)| no_out!(limbs_sub_same_length_to_out(&mut xs, &ys, &zs))),
        )],
    );
}

fn benchmark_limbs_sub_to_out(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_to_out(&mut [Limb], &[Limb], &[Limb])",
        BenchmarkType::Single,
        triples_of_unsigned_vec_var_9(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, ref ys, ref zs)| max(ys.len(), zs.len())),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys, zs)| no_out!(limbs_sub_to_out(&mut xs, &ys, &zs))),
        )],
    );
}

fn benchmark_limbs_sub_same_length_in_place_left(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_sub_same_length_in_place_left(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys)| no_out!(limbs_sub_same_length_in_place_left(&mut xs, &ys))),
        )],
    );
}

fn benchmark_limbs_sub_in_place_left(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_sub_in_place_left(&Vec<Limb>, &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(mut xs, ys)| no_out!(limbs_sub_in_place_left(&mut xs, &ys))),
        )],
    );
}

fn benchmark_limbs_sub_same_length_in_place_right(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_sub_same_length_in_place_right(&mut [Limb], &[Limb])",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len() = ys.len()",
        &mut [(
            "malachite",
            &mut (|(xs, mut ys)| no_out!(limbs_sub_same_length_in_place_right(&xs, &mut ys))),
        )],
    );
}

fn benchmark_limbs_slice_sub_in_place_right(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_slice_sub_in_place_right(&[Limb], &mut [Limb], usize)",
        BenchmarkType::Single,
        triples_of_unsigned_vec_unsigned_and_small_usize_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys, _)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(xs, mut ys, len)| no_out!(limbs_slice_sub_in_place_right(&xs, &mut ys, len))),
        )],
    );
}

fn benchmark_limbs_vec_sub_in_place_right(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "limbs_vec_sub_in_place_right(&[Limb], &mut Vec<Limb>)",
        BenchmarkType::Single,
        pairs_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, ref ys)| min(xs.len(), ys.len())),
        "min(xs.len(), ys.len())",
        &mut [(
            "malachite",
            &mut (|(xs, mut ys)| no_out!(limbs_vec_sub_in_place_right(&xs, &mut ys))),
        )],
    );
}

fn benchmark_limbs_sub_same_length_in_place_with_overlap_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_sub_same_length_in_place_with_overlap(&mut [Limb], usize)",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_and_small_usize_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "standard",
                &mut (|(mut xs, right_start)| {
                    no_out!(limbs_sub_same_length_in_place_with_overlap(
                        &mut xs,
                        right_start
                    ))
                }),
            ),
            (
                "naive",
                &mut (|(mut xs, right_start)| {
                    no_out!(limbs_sub_same_length_in_place_with_overlap_naive(
                        &mut xs,
                        right_start
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_limbs_sub_same_length_to_out_with_overlap_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "limbs_sub_same_length_to_out_with_overlap(&mut [Limb], usize)",
        BenchmarkType::Algorithms,
        pairs_of_unsigned_vec_var_3(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref xs, _)| xs.len()),
        "xs.len()",
        &mut [
            (
                "standard",
                &mut (|(mut xs, ys)| {
                    no_out!(limbs_sub_same_length_to_out_with_overlap(&mut xs, &ys))
                }),
            ),
            (
                "naive",
                &mut (|(mut xs, ys)| {
                    no_out!(limbs_sub_same_length_to_out_with_overlap_naive(
                        &mut xs, &ys
                    ))
                }),
            ),
        ],
    );
}

fn benchmark_natural_sub_assign_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural -= Natural",
        BenchmarkType::LibraryComparison,
        rm_pairs_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref x, ref y))| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x -= y)),
            ("rug", &mut (|((mut x, y), _)| x -= y)),
        ],
    );
}

fn benchmark_natural_sub_assign_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural -= Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural -= Natural", &mut (|(mut x, y)| x -= y)),
            ("Natural -= &Natural", &mut (|(mut x, y)| x -= &y)),
        ],
    );
}

fn benchmark_natural_sub_library_comparison(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural - Natural",
        BenchmarkType::LibraryComparison,
        nrm_pairs_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, _, (ref x, ref y))| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("malachite", &mut (|(_, _, (x, y))| no_out!(x - y))),
            ("num", &mut (|((x, y), _, _)| no_out!(x - y))),
            ("rug", &mut (|(_, (x, y), _)| no_out!(x - y))),
        ],
    );
}

fn benchmark_natural_sub_evaluation_strategy(gm: GenerationMode, limit: usize, file_name: &str) {
    m_run_benchmark(
        "Natural - Natural",
        BenchmarkType::EvaluationStrategy,
        pairs_of_naturals_var_1(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref x, ref y)| {
            usize::checked_from(max(x.significant_bits(), y.significant_bits())).unwrap()
        }),
        "max(x.significant_bits(), y.significant_bits())",
        &mut [
            ("Natural - Natural", &mut (|(x, y)| no_out!(x - y))),
            ("Natural - &Natural", &mut (|(x, y)| no_out!(x - &y))),
            ("&Natural - Natural", &mut (|(x, y)| no_out!(&x - y))),
            ("&Natural - &Natural", &mut (|(x, y)| no_out!(&x - &y))),
        ],
    );
}
