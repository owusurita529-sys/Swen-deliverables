// -------------------------------------------------------
// speed_race.rs
// Student: [RITA OWUSU]
// Benchmarks branch_counter (DFS) vs queue_counter (BFS)
// Run with: cargo bench
// -------------------------------------------------------

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use subordinates::{branch_counter, queue_counter};

// 🏗️ Build a chain tree of given size (worst case for DFS)
fn make_chain(n: usize) -> Vec<Vec<usize>> {
    let mut org_chart = vec![vec![]; n + 1];
    for i in 1..n {
        org_chart[i].push(i + 1);
        org_chart[i + 1].push(i);
    }
    org_chart
}

// ⏱️ Benchmark APPROACH A — Recursive DFS (branch_counter)
fn bench_branch_counter(c: &mut Criterion) {
    let org_chart = make_chain(1000);
    c.bench_function("branch_counter DFS n=1000", |b| {
        b.iter(|| {
            let mut tally = vec![0usize; 1001];
            branch_counter(
                black_box(1),
                black_box(0),
                black_box(&org_chart),
                &mut tally,
            );
            tally
        })
    });
}

// ⏱️ Benchmark APPROACH B — Iterative BFS (queue_counter)
fn bench_queue_counter(c: &mut Criterion) {
    let org_chart = make_chain(1000);
    c.bench_function("queue_counter BFS n=1000", |b| {
        b.iter(|| {
            queue_counter(black_box(1000), black_box(&org_chart))
        })
    });
}

criterion_group!(benches, bench_branch_counter, bench_queue_counter);
criterion_main!(benches);