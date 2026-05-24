// -------------------------------------------------------
// speed_race.rs
// Student: [RITA OWUSU]
// Benchmarks ribbon_tracer (KMP) vs ribbon_checker (Naive)
// Run with: cargo bench
// -------------------------------------------------------

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use finding_borders::{ribbon_tracer, ribbon_checker};

// 🏗️ Build a worst-case string — all same letters!
// e.g. "aaaa...a" (1000 times)
// This maximises borders and makes naive work hardest!
fn make_test_word(size: usize) -> Vec<u8> {
    vec![b'a'; size]
}

// ⏱️ Benchmark APPROACH A — KMP (ribbon_tracer)
fn bench_ribbon_tracer(c: &mut Criterion) {
    let word = make_test_word(1000);
    c.bench_function("ribbon_tracer KMP n=1000", |b| {
        b.iter(|| ribbon_tracer(black_box(&word)))
    });
}

// ⏱️ Benchmark APPROACH B — Naive (ribbon_checker)
fn bench_ribbon_checker(c: &mut Criterion) {
    let word = make_test_word(1000);
    c.bench_function("ribbon_checker Naive n=1000", |b| {
        b.iter(|| ribbon_checker(black_box(&word)))
    });
}

criterion_group!(benches, bench_ribbon_tracer, bench_ribbon_checker);
criterion_main!(benches);