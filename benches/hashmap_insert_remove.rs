use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut hashmap1000 = (0..1000).into_iter().map(|i| (i, i)).collect::<HashMap<u32, u32>>();
    c.bench_function("remove + insert", |b| {
        b.iter(|| {
            hashmap1000.remove(&896);
            hashmap1000.insert(896, 896);
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);