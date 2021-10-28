use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let vec10 = (0..10).into_iter().collect::<Vec<u32>>();
    let vec100 = (0..100).into_iter().collect::<Vec<u32>>();
    let vec1000 = (0..1000).into_iter().collect::<Vec<u32>>();
    let hashmap10 = (0..10).into_iter().map(|i| (i, i)).collect::<HashMap<u32, u32>>();
    let hashmap100 = (0..100).into_iter().map(|i| (i, i)).collect::<HashMap<u32, u32>>();
    let hashmap1000 = (0..1000).into_iter().map(|i| (i, i)).collect::<HashMap<u32, u32>>();
    c.bench_function("vec 10 find", |b| {
        b.iter(|| vec10.iter().find(|i| **i == 9).unwrap());
    });
    c.bench_function("hashmap 10 get", |b| {
        b.iter(|| hashmap10.get(&9).unwrap());
    });
    c.bench_function("vec 100 find", |b| {
        b.iter(|| vec100.iter().find(|i| **i == 99).unwrap());
    });
    c.bench_function("hashmap 100 get", |b| {
        b.iter(|| hashmap100.get(&99).unwrap());
    });
    c.bench_function("vec 1000 find", |b| {
        b.iter(|| vec1000.iter().find(|i| **i == 999).unwrap());
    });
    c.bench_function("hashmap 1000 get", |b| {
        b.iter(|| hashmap1000.get(&999).unwrap());
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);