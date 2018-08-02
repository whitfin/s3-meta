//! General metrics tracking for S3 objects.
use humantime;
use rusoto_s3::Object;
use std::collections::HashSet;
use std::path::Path;
use std::time::{Duration, SystemTime};

use super::Metric;

/// Container struct for general metrics tracked by S3.
pub struct General {
    folder_set: HashSet<String>,
    prefix_len: usize,
    start_time: SystemTime,
    total_keys: u64,
    total_size: u64,
}

/// Main implementation.
impl General {
    /// Constructs a new `General` struct.
    pub(super) fn new(prefix: &Option<String>) -> General {
        General {
            folder_set: HashSet::new(),
            prefix_len: prefix.as_ref().map(|s| s.len()).unwrap_or(1) - 1,
            start_time: SystemTime::now(),
            total_keys: 0,
            total_size: 0,
        }
    }
}

/// Metric implementation.
impl Metric for General {
    /// Registers an S3 `Object` with this metric struct.
    fn register(&mut self, object: &Object) {
        // grab the key of the object
        let key = super::get_key(object);

        // walk the ancestors, skipping the file name
        for dir in Path::new(&key).ancestors().skip(1) {
            let path = dir.to_string_lossy();

            // skip anything above/or the root
            if path.len() <= self.prefix_len {
                continue;
            }

            // trim off the configured prefix
            let trimmed = &path[self.prefix_len..];
            self.folder_set.insert(trimmed.to_string());
        }

        // increment counters
        self.total_keys += 1;
        self.total_size += super::get_size(object);
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
        ::util::log_pair("total_files", self.total_keys);
        ::util::log_pair("total_folders", self.folder_set.len());
        ::util::log_pair("total_storage", ::util::convert_bytes(self.total_size));
    }
}
