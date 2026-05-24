use criterion::{black_box, criterion_group, criterion_main, Criterion};
use array_lifter::lift_blocks;

fn make_test_stones(size: usize) -> Vec<i64> {
    (0..size).map(|i| (size - i) as i64).collect()
}

fn bench_lift_blocks(c: &mut Criterion) {
    let stones = make_test_stones(1000);
    c.bench_function("lift_blocks recursive 1000", |b| {
        b.iter(|| lift_blocks(black_box(&stones), 0, 0, 0))
    });
}

criterion_group!(benches, bench_lift_blocks);
criterion_main!(benches);