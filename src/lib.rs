use csv::ReaderBuilder;
use flate2::read::GzDecoder;
use super_mass::{
    mass_batch,
    time_series::{self, Record, TimeSeries},
};

use std::fs::File;
pub use std::io::Error as IOError;
use std::vec::Vec;
use tar::{self, Archive};

pub fn untar_gz(path: &str) -> Result<Archive<GzDecoder<File>>, IOError> {
    let tar = File::open(path)?;
    let tar = GzDecoder::new(tar);
    Ok(Archive::new(tar))
}

pub fn load_series<T>(
    mut archive: Archive<GzDecoder<File>>,
    take_this_many: Option<usize>,
) -> Result<time_series::TimeSeries<f64>, IOError> {
    let file = archive.entries()?.next().unwrap()?;

    // read file
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    // collect into vector
    let result: Vec<Record<f64>>;
    let column = reader
        .deserialize()
        .map(|x: Result<Record<f64>, _>| x.unwrap());

    if let Some(n) = take_this_many {
        result = column.take(n).collect();
    } else {
        result = column.collect();
    }

    let series = time_series::TimeSeries::new(result);
    Ok(series)
}

/// Load samples from time series and query.
pub fn input_pair(
    n: Option<usize>,
    q: Option<usize>,
    ts_path: &str,
    query_path: &str,
) -> Result<(TimeSeries<f64>, TimeSeries<f64>), IOError> {
    let archive = untar_gz(ts_path)?;
    let ts = load_series::<f64>(archive, n)?;

    let archive = untar_gz(query_path)?;
    let query = load_series::<f64>(archive, q)?;
    return Ok((ts, query));
}

static QUERY_PATH: &str = "data/ecg_query.tar.gz";
static TS_PATH: &str = "data/ecg.tar.gz";

///short example on the mass method over a subsample of a time series and a query.
pub fn short() -> Result<Vec<(usize, f64)>, IOError> {
    {
        const N: usize = 100;
        const Q: usize = 10;

        let (ts, query) = input_pair(Some(N), Some(Q), TS_PATH, QUERY_PATH).unwrap();

        let ts = &ts.series[..];
        let query = &query.series[..];

        let batch_size = N / 2;
        let top_matches = 1;

        let top = mass_batch(ts, query, batch_size, top_matches);

        Ok(top)
    }
}

pub fn full() -> Result<Vec<(usize, f64)>, IOError> {
    let (ts, query) = input_pair(None, None, TS_PATH, QUERY_PATH).unwrap();

    let query = &query.series[..];
    let ts = &ts.series[..];

    let batch_size = 10000;
    let top_matches = 4;
    let top = mass_batch(ts, query, batch_size, top_matches);
    Ok(top)
}

#[cfg(test)]
mod tests {

    use super::*;
    use itertools::assert_equal;
    #[test]
    fn short() -> Result<(), IOError> {
        let top = super::short()?;
        print!("{:?}", top);
        assert!(top[0].0 == 76);
        Ok(())
    }

    #[test]
    #[ignore = "Takes too long for quick tests"]
    fn full() -> Result<(), IOError> {
        let top = super::full()?;
        let (a, _): (Vec<_>, Vec<_>) = top.iter().copied().unzip();
        assert_equal(a, vec![165429, 1078710, 19859632, 1547214]);
        Ok(())
    }
}
