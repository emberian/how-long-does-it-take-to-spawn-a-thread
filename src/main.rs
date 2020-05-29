use criterion::*;

use std::{mem, thread, time};

fn bench_thread_spawn(c: &mut Criterion) {
    c.bench_function("tspawn", |b| b.iter_with_large_drop(|| thread::spawn(|| ())));
}

criterion_group!(benches, bench_thread_spawn);
criterion_main!(benches);
