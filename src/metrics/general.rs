//! General metrics tracking for S3 objects.
use humantime;
use rusoto_s3::Object;
use std::time::{Duration, SystemTime};

use super::Metric;

/// Container struct for general metrics tracked by S3.
pub struct General {
    start_time: SystemTime,
    total_keys: u64,
    total_size: u64,
}

/// Metric implementation.
impl Metric for General {
    /// Constructs a new `General` struct.
    fn new() -> General {
        General {
            start_time: SystemTime::now(),
            total_keys: 0,
            total_size: 0,
        }
    }

    /// Registers an S3 `Object` with this metric struct.
    fn register(&mut self, object: &Object) {
        self.total_keys += 1;
        self.total_size += object.size.unwrap() as u64;
    }

    /// Prints out all internal statistics under the `general` header.
    fn print(&self) {
        // task done, so check execution time
        let task_duration = Duration::from_secs(
            SystemTime::now()
                .duration_since(self.start_time)
                .expect("SystemTime::duration_since failed")
                .as_secs(),
        );

        // initial header!
        println!("[general]");

        // log out the total time, total space, and total file count
        ::util::log_pair("total_time", humantime::format_duration(task_duration));
        ::util::log_pair("total_space", ::util::convert_bytes(self.total_size));
        ::util::log_pair("total_files", ::util::comma(self.total_keys));
    }
}
