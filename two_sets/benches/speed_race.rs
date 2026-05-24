// -------------------------------------------------------
// speed_race.rs
// Student: [RITA OWUSU]
// Benchmarks tray_filler (greedy) vs tray_builder
// (mathematical) for Two Sets
// Run with: cargo bench
// -------------------------------------------------------

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use two_sets::{tray_filler, tray_builder};

// ⏱️ Benchmark APPROACH A — Greedy (tray_filler)
fn bench_tray_filler(c: &mut Criterion) {
    c.bench_function("tray_filler greedy n=1000", |b| {
        b.iter(|| tray_filler(black_box(1000)))
    });
}

// ⏱️ Benchmark APPROACH B — Mathematical (tray_builder)
fn bench_tray_builder(c: &mut Criterion) {
    c.bench_function("tray_builder mathematical n=1000", |b| {
        b.iter(|| tray_builder(black_box(1000)))
    });
}

criterion_group!(benches, bench_tray_filler, bench_tray_builder);
criterion_main!(benches);