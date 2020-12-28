use criterion::{criterion_group, criterion_main, Criterion};

use mass_bench::input_pair;
use super_mass::mass_batch;

pub fn criterion_benchmark(c: &mut Criterion) {
    const N: usize = 100_000;
    const Q: usize = 10;

    let (ts, query) =
        input_pair(Some(N), Some(Q), "data/ecg.tar.gz", "data/ecg_query.tar.gz").unwrap();

    let query = &query.series[..];
    let ts = &ts.series[..];

    let batch_size = N / 16;
    let top_matches = 10;
    let jobs = 8usize;

    super_mass::init_pool(jobs);

    c.bench_function("Short Bench", |b| {
        b.iter(|| mass_batch(ts, query, batch_size, top_matches))
    });
}

criterion_group!(benches, criterion_benchmark);

criterion_main!(benches);
