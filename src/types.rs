//! Types module for the main runtime, exposing error and result types.
use quick_xml::events::Event;
use quick_xml::Reader;
use rusoto_s3::ListObjectsV2Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::{io, time};

/// Public type alias for a result with a `MetaError` error type.
pub type MetaResult<T> = Result<T, MetaError>;

/// Delegating error wrapper for errors raised by the main archive.
///
/// The internal `String` representation enables cheap coercion from
/// other error types by binding their error messages through. This
/// is somewhat similar to the `failure` crate, but minimal.
pub struct MetaError(String);

/// Debug implementation for `MetaError`.
impl Debug for MetaError {
    /// Formats an `MetaError` by delegating to `Display`.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self, f)
    }
}

/// Display implementation for `MetaError`.
impl Display for MetaError {
    /// Formats an `MetaError` by writing out the inner representation.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Error translation from Rusoto to relay messages.
impl From<ListObjectsV2Error> for MetaError {
    /// Converts a `ListObjectsV2Error` to a `MetaError`.
    fn from(err: ListObjectsV2Error) -> MetaError {
        // grab the raw conversion
        let msg = err.to_string();

        // XML, look for a message!
        if msg.starts_with("<?xml") {
            // create an XML reader and buffer
            let mut reader = Reader::from_str(&msg);
            let mut buffer = Vec::new();

            loop {
                // parse through each XML node event
                match reader.read_event(&mut buffer) {
                    // end, or error, just give up
                    Ok(Event::Eof) | Err(_) => break,

                    // if we find a message tag, we'll use that as the error
                    Ok(Event::Start(ref e)) if e.name() == b"Message" => {
                        return MetaError(
                            reader
                                .read_text(b"Message", &mut Vec::new())
                                .expect("Cannot decode text value"),
                        )
                    }

                    // skip
                    _ => (),
                }
                // empty buffers
                buffer.clear();
            }
        }

        // default msg
        MetaError(msg)
    }
}

/// Macro to implement `From` for provided types.
macro_rules! derive_from {
    ($type:ty) => {
        impl<'a> From<$type> for MetaError {
            fn from(t: $type) -> MetaError {
                MetaError(t.to_string())
            }
        }
    };
}

// Easy derivations of derive_from.
derive_from!(&'a str);
derive_from!(io::Error);
derive_from!(time::SystemTimeError);
derive_from!(String);
