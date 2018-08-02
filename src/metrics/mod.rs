//! Parent metric module exposing traits around metrics gathering.
use rusoto_s3::Object;

pub mod extensions;
pub mod file_size;
pub mod general;
pub mod modification;

use self::extensions::Extensions;
use self::file_size::FileSize;
use self::general::General;
use self::modification::Modification;

/// Metric trait to represent a metric tracker for S3.
///
/// Implementing this trait means that the structure can be used to
/// track metrics on objects stored in S3. Object instances will be
/// fed through to `register` on each entry in S3.
pub trait Metric {
    /// Creates a new instance of this structure.
    fn new() -> Self
    where
        Self: Sized;

    /// Registers an S3 object for statistics.
    fn register(&mut self, object: &Object);

    /// Prints the internal statistics.
    fn print(&self);
}

/// Returns a chain of `Metric` objects in deterministic order.
pub fn chain() -> Vec<Box<Metric>> {
    vec![
        Box::new(General::new()),
        Box::new(FileSize::new()),
        Box::new(Extensions::new()),
        Box::new(Modification::new()),
    ]
}
