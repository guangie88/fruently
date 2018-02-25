//! Retry sending records configuration.

use std::path::PathBuf;

/// You can calculate retrying interval as the following equation:
///
/// `retry_interval = exp ** (multiplier + retry_counts)`
///
/// see: <https://github.com/jimmycuadra/retry/blob/v0.4.0/src/lib.rs#L142-L143>
///
/// You can estimate to caluculate with concrete values like:
///
/// * `retry_counts`: 10, e ^ (5 + 10)/1000.0/60.0/60.0 = 0.908060381242253, about 0.9 hour
///
/// * `retry_counts`: 11, e ^ (5 + 11)/1000.0/60.0/60.0 = 2.4683640334744092, about 2.5 hours
///
/// * `retry_counts`: 12, e ^ (5 + 12)/1000.0/60.0/60.0 = 6.709709098215361, about 6.7 hours
///
/// where multiplier = 5,
/// e is [exponential function](https://doc.rust-lang.org/std/primitive.f64.html#method.exp).
///
/// If you specify `store_file_path`, `fruently` tries to store record(s)
/// when failing to send into Fluent protocol implemented server.
///
/// ## Default values
///
/// * multiplier: `5_f64`
/// * max: 10
/// * `store_file_path`: None
#[derive(Debug, Clone, PartialEq)]
pub struct RetryConf {
    max: u64,
    multiplier: f64,
    store_file_path: Option<PathBuf>,
}

impl Default for RetryConf {
    fn default() -> RetryConf {
        RetryConf {
            max: 10,
            multiplier: 5_f64,
            store_file_path: None,
        }
    }
}

impl RetryConf {
    pub fn new() -> RetryConf {
        Default::default()
    }

    pub fn max(mut self, max: u64) -> RetryConf {
        self.max = max;
        self
    }

    pub fn multiplier(mut self, multiplier: f64) -> RetryConf {
        self.multiplier = multiplier;
        self
    }

    pub fn store_file(mut self, path: PathBuf) -> RetryConf {
        self.store_file_path = Some(path);
        self
    }

    pub fn need_to_store(self) -> bool {
        self.store_file_path.is_some()
    }

    pub fn store_path(self) -> Option<PathBuf> {
        self.store_file_path
    }

    pub fn build(self) -> (u64, f64) {
        (self.max, self.multiplier)
    }
}
