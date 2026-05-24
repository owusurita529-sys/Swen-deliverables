// -------------------------------------------------------
// speed_race.rs
// Student: [YOUR NAME]
// Benchmarks ring_shifter (recursive) vs ring_machine
// (iterative) for Tower of Hanoi
// Run with: cargo bench
// -------------------------------------------------------

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hanoi_solver1::{ring_shifter, ring_machine};

// ⏱️ Benchmark APPROACH A — Recursive (ring_shifter)
fn bench_ring_shifter(c: &mut Criterion) {
    c.bench_function("ring_shifter recursive n=15", |b| {
        b.iter(|| {
            let mut log = Vec::new();
            ring_shifter(black_box(15), "Left", "Right", "Middle", &mut log);
            log
        })
    });
}

// ⏱️ Benchmark APPROACH B — Iterative (ring_machine)
fn bench_ring_machine(c: &mut Criterion) {
    c.bench_function("ring_machine iterative n=15", |b| {
        b.iter(|| {
            ring_machine(black_box(15), "Left", "Right", "Middle")
        })
    });
}

criterion_group!(benches, bench_ring_shifter, bench_ring_machine);
criterion_main!(benches);