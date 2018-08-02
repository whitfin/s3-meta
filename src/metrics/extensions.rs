//! Extension
//!  metrics tracking for S3 objects.
use rusoto_s3::Object;
use std::collections::HashMap;
use std::path::Path;

use super::Metric;

/// Container struct for extension metrics tracked by S3.
pub struct Extensions {
    extensions: HashMap<String, u64>,
}

/// Metric implementation.
impl Metric for Extensions {
    /// Constructs a new `Extensions` struct.
    fn new() -> Extensions {
        Extensions {
            extensions: HashMap::new(),
        }
    }

    /// Registers an S3 `Object` with this metric struct.
    fn register(&mut self, object: &Object) {
        // grab the file extensions and increment
        if let Some(ext) = Path::new(object.key.as_ref().unwrap()).extension() {
            *self.extensions
                .entry(ext.to_string_lossy().into_owned())
                .or_insert(0) += 1;
        }
    }

    /// Prints out all internal statistics under the `extensions` header.
    fn print(&self) {
        // next segment: extensions
        ::util::log_head("extensions");
        ::util::log_pair("unique_extensions", self.extensions.len());

        // find the most frequent extension
        let prevalent_extension = self.extensions
            .iter()
            .max_by(|(_, left), (_, right)| left.cmp(right));

        // log out a potential most frequent
        if let Some((ext, _)) = prevalent_extension {
            ::util::log_pair("most_popular_extension", ext);
        }
    }
}
