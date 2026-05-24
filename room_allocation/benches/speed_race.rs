use criterion::{black_box, criterion_group, criterion_main, Criterion};
use room_allocation::{hotel_desk, checkout_queue};

fn make_bookings(size: usize) -> Vec<(i64, i64)> {
    (0..size)
        .map(|i| (i as i64, (i + size) as i64))
        .collect()
}

fn bench_hotel_desk(c: &mut Criterion) {
    let bookings = make_bookings(1000);
    c.bench_function("hotel_desk BTreeMap n=1000", |b| {
        b.iter(|| hotel_desk(black_box(&bookings)))
    });
}

fn bench_checkout_queue(c: &mut Criterion) {
    let bookings = make_bookings(1000);
    c.bench_function("checkout_queue BinaryHeap n=1000", |b| {
        b.iter(|| checkout_queue(black_box(&bookings)))
    });
}

criterion_group!(benches, bench_hotel_desk, bench_checkout_queue);
criterion_main!(benches);