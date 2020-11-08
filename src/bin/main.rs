use mass::mass_batch;
use mass_bench::input_pair;

fn main() {
    let (ts, query) =
        input_pair(None, None, "python/ecg.tar.gz", "python/ecg_query.tar.gz").unwrap();

    let query = &query.series[..];
    let ts = &ts.series[..];

    let batch_size = 10000;
    let top_matches = 4;
    let jobs = 1;

    let _top = mass_batch(ts, query, batch_size, top_matches, jobs);
}
