use common::{natural_to_biguint, natural_to_rug_integer, GenerationMode};
use inputs::natural::{pairs_of_natural_and_unsigned, pairs_of_unsigned_and_natural};
use malachite_base::num::SignificantBits;
use num::BigUint;
use rug;
use rust_wheels::benchmarks::{BenchmarkOptions2, BenchmarkOptions3, benchmark_2, benchmark_3};

#[allow(unknown_lints, assign_op_pattern)]
pub fn num_mul_u32(mut x: BigUint, u: u32) -> BigUint {
    x = x * BigUint::from(u);
    x
}

pub fn demo_natural_mul_assign_u32(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        n *= u;
        println!("x := {}; x *= {}; x = {}", n_old, u, n);
    }
}

pub fn demo_natural_mul_u32(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * {} = {}", n_old, u, n * u);
    }
}

pub fn demo_natural_mul_u32_ref(gm: GenerationMode, limit: usize) {
    for (n, u) in pairs_of_natural_and_unsigned::<u32>(gm).take(limit) {
        println!("&{} * {} = {}", n, u, &n * u);
    }
}

pub fn demo_u32_mul_natural(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * {} = {}", u, n_old, u * n);
    }
}

pub fn demo_u32_mul_natural_ref(gm: GenerationMode, limit: usize) {
    for (u, n) in pairs_of_unsigned_and_natural::<u32>(gm).take(limit) {
        let n_old = n.clone();
        println!("{} * &{} = {}", u, n_old, u * &n);
    }
}

pub fn benchmark_natural_mul_assign_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural *= u32", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_natural_and_unsigned::<u32>(gm),
        function_f: &(|(mut n, u)| n *= u),
        function_g: &(|(mut n, u): (rug::Integer, u32)| n *= u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (natural_to_rug_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "Natural *= u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_mul_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Natural * u32", gm.name());
    benchmark_3(BenchmarkOptions3 {
        xs: pairs_of_natural_and_unsigned::<u32>(gm),
        function_f: &(|(n, u)| n * u),
        function_g: &(|(n, u): (BigUint, u32)| num_mul_u32(n, u)),
        function_h: &(|(n, u): (rug::Integer, u32)| n * u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(ref n, u)| (natural_to_biguint(n), u)),
        z_cons: &(|&(ref n, u)| (natural_to_rug_integer(n), u)),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "num",
        h_name: "rug",
        title: "Natural * u32",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_natural_mul_u32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Natural * u32 evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_natural_and_unsigned::<u32>(gm),
        function_f: &(|(n, u)| n * u),
        function_g: &(|(n, u)| &n * u),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(ref n, _)| n.significant_bits() as usize),
        limit,
        f_name: "Natural * u32",
        g_name: "\\\\&Natural * u32",
        title: "Natural * u32 evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_u32_mul_natural(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} u32 * Natural", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_unsigned_and_natural::<u32>(gm),
        function_f: &(|(u, n)| u * n),
        function_g: &(|(u, n): (u32, rug::Integer)| u * n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|&(u, ref n)| (u, natural_to_rug_integer(n))),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rug",
        title: "u32 * Natural",
        x_axis_label: "other",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_u32_mul_natural_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} u32 * Natural evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: pairs_of_unsigned_and_natural::<u32>(gm),
        function_f: &(|(u, n)| u * n),
        function_g: &(|(u, n)| u * &n),
        x_cons: &(|p| p.clone()),
        y_cons: &(|p| p.clone()),
        x_param: &(|&(_, ref n)| n.significant_bits() as usize),
        limit,
        f_name: "u32 * Natural",
        g_name: "u32 * \\\\&Natural",
        title: "u32 * Natural evaluation strategy",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
