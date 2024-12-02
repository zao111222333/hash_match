#[expect(dead_code)]
mod demo_bench_5;

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub const BENCHS: &[(usize, fn())] = &[
    (5, bench_5::benches),
    (10, bench_10::benches),
    (15, bench_15::benches),
    (20, bench_20::benches),
    (35, bench_35::benches),
    (50, bench_50::benches),
    (75, bench_75::benches),
    (100, bench_100::benches),
    (200, bench_200::benches),
    (500, bench_500::benches),
    (1000, bench_1000::benches),
    (2000, bench_2000::benches),
    (5000, bench_5000::benches),
    (10000, bench_10000::benches),
];
