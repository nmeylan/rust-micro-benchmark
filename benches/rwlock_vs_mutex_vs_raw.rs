use std::sync::{Mutex, RwLock};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let rwlock = RwLock::new(99_u32);
    let mutex = Mutex::new(99_u32);
    let raw = 99_u32;
    c.bench_function("rwlock", |b| {
        b.iter(|| {
            let guard = rwlock.read().unwrap();
            let value = *guard + 1;
            value
        });
    });
    c.bench_function("mutex", |b| {
        b.iter(|| {
            let guard = mutex.lock().unwrap();
            let value = *guard + 1;
            value
        });
    });
    c.bench_function("raw", |b| {
        b.iter(|| {
            let value = raw + 1;
            value
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);