use criterion::{criterion_group, criterion_main, Criterion};

use mass::mass_batch;
use mass_bench::input_pair;

pub fn criterion_benchmark(c: &mut Criterion) {
    const N: usize = 100_000;
    const Q: usize = 10;

    let (ts, query) = input_pair(
        Some(N),
        Some(Q),
        "python/ecg.tar.gz",
        "python/ecg_query.tar.gz",
    )
    .unwrap();

    let query = &query.series[..];
    let ts = &ts.series[..];

    let batch_size = N / 4;
    // let batch_size = N;
    let top_matches = 10;
    let jobs = 1;

    c.bench_function("Short Bench", |b| {
        b.iter(|| mass_batch(ts, query, batch_size, top_matches, jobs))
    });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
