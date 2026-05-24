// -------------------------------------------------------
// speed_race.rs
// Student: [RITA OWUSU]
// Benchmarks ring_counter (BTreeSet) vs
// champion_finder (formula) for Josephus Problem I
// Run with: cargo bench
// -------------------------------------------------------

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use josephus::{ring_counter, champion_finder};

// ⏱️ Benchmark APPROACH A — BTreeSet simulation
fn bench_ring_counter(c: &mut Criterion) {
    c.bench_function("ring_counter BTreeSet n=1000", |b| {
        b.iter(|| ring_counter(black_box(1000)))
    });
}

// ⏱️ Benchmark APPROACH B — Mathematical formula
fn bench_champion_finder(c: &mut Criterion) {
    c.bench_function("champion_finder formula n=1000", |b| {
        b.iter(|| champion_finder(black_box(1000)))
    });
}

criterion_group!(benches, bench_ring_counter, bench_champion_finder);
criterion_main!(benches);