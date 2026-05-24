// -------------------------------------------------------
// speed_race.rs
// Student: [RITA OWUSU]
// Benchmarks count_with_diver (DFS) vs
// count_with_sweeper (BFS) for Counting Rooms
// Run with: cargo bench
// -------------------------------------------------------

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use counting_rooms::{count_with_diver, count_with_sweeper};

// 🏗️ Build a big checkerboard map — lots of tiny rooms!
fn make_checkerboard(size: usize) -> Vec<Vec<char>> {
    (0..size)
        .map(|r| {
            (0..size)
                .map(|c| if (r + c) % 2 == 0 { '.' } else { '#' })
                .collect()
        })
        .collect()
}

// ⏱️ Benchmark APPROACH A — Recursive DFS
fn bench_flood_diver(c: &mut Criterion) {
    let blueprint = make_checkerboard(100);
    c.bench_function("count_with_diver DFS 100x100", |b| {
        b.iter(|| count_with_diver(black_box(&blueprint)))
    });
}

// ⏱️ Benchmark APPROACH B — Iterative BFS
fn bench_flood_sweeper(c: &mut Criterion) {
    let blueprint = make_checkerboard(100);
    c.bench_function("count_with_sweeper BFS 100x100", |b| {
        b.iter(|| count_with_sweeper(black_box(&blueprint)))
    });
}

criterion_group!(benches, bench_flood_diver, bench_flood_sweeper);
criterion_main!(benches);