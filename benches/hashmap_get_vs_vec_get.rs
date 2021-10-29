use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let vec1000 = (0..1000).into_iter().collect::<Vec<u32>>();
    let hashmap1000 = (0..1000).into_iter().map(|i| (i, i)).collect::<HashMap<u32, u32>>();
    let vec160000 = (0..160000).into_iter().collect::<Vec<u32>>();
    let hashmap160000 = (0..160000).into_iter().map(|i| (i, i)).collect::<HashMap<u32, u32>>();

    c.bench_function("vec 1000 find", |b| {
        b.iter(|| vec1000.get(black_box(695)).unwrap());
    });
    c.bench_function("hashmap 1000 get", |b| {
        b.iter(|| hashmap1000.get(&black_box(695)).unwrap());
    });
    c.bench_function("vec 160000 find", |b| {
        b.iter(|| vec160000.get(black_box(154698)).unwrap());
    });
    c.bench_function("hashmap 160000 get", |b| {
        b.iter(|| hashmap160000.get(&black_box(154698)).unwrap());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);