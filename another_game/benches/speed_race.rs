// -------------------------------------------------------
// speed_race.rs
// Student: [RITA OWUSU]
// Benchmarks light_seeker vs light_merger
// Run with: cargo bench
// -------------------------------------------------------

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use another_game::{light_seeker, light_merger};

// 🏗️ Build worst case — all even piles (no early exit!)
// light_seeker must check every single pile!
fn make_even_piles(size: usize) -> Vec<u64> {
    (1..=size).map(|i| (i * 2) as u64).collect()
}

// ⏱️ Benchmark APPROACH A — Early exit loop
fn bench_light_seeker(c: &mut Criterion) {
    let coin_piles = make_even_piles(1000);
    c.bench_function("light_seeker early_exit n=1000", |b| {
        b.iter(|| light_seeker(black_box(&coin_piles)))
    });
}

// ⏱️ Benchmark APPROACH B — Bitwise fold
fn bench_light_merger(c: &mut Criterion) {
    let coin_piles = make_even_piles(1000);
    c.bench_function("light_merger bitwise_fold n=1000", |b| {
        b.iter(|| light_merger(black_box(&coin_piles)))
    });
}

criterion_group!(benches, bench_light_seeker, bench_light_merger);
criterion_main!(benches);