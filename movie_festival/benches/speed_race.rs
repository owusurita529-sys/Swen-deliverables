// -------------------------------------------------------
// speed_race.rs
// Student: [RITA OWUSU]
// Benchmarks screen_scheduler vs film_sweeper
// Run with: cargo bench
// -------------------------------------------------------

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use movie_festival::{screen_scheduler, film_sweeper};

// 🏗️ Build worst case — lots of overlapping films
fn make_screenings(size: usize) -> Vec<(i64, i64)> {
    (0..size)
        .map(|i| (i as i64, (i + size) as i64))
        .collect()
}

// ⏱️ Benchmark APPROACH A — Sort + Greedy
fn bench_screen_scheduler(c: &mut Criterion) {
    let screenings = make_screenings(1000);
    c.bench_function("screen_scheduler sort+greedy n=1000", |b| {
        b.iter(|| screen_scheduler(black_box(&screenings)))
    });
}

// ⏱️ Benchmark APPROACH B — Min-heap sweep
fn bench_film_sweeper(c: &mut Criterion) {
    let screenings = make_screenings(1000);
    c.bench_function("film_sweeper heap_sweep n=1000", |b| {
        b.iter(|| film_sweeper(black_box(&screenings)))
    });
}

criterion_group!(benches, bench_screen_scheduler, bench_film_sweeper);
criterion_main!(benches);