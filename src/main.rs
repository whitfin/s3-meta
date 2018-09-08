//! Metadata gathering for S3 buckets and subsections.
//!
//! This tool should be used from a command line and can be used to output
//! a report about an Amazon S3 bucket, or subsection thereof.
//!
//! Credentials must be provided via guidelines in the [AWS Documentation]
//! (https://docs.aws.amazon.com/cli/latest/userguide/cli-environment.html).
extern crate humantime;
extern crate pretty_bytes;
extern crate quick_xml;
extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_core::{credential::ChainProvider, region::Region, HttpClient};
use rusoto_s3::{ListObjectsV2Request, S3, S3Client};
use std::time::Duration;

mod bounded;
mod metrics;
mod types;
mod util;

fn main() -> types::MetaResult<()> {
    // grab the root path of the S3 location to use
    let root_paths = std::env::args()
        .skip(1)
        .next()
        .ok_or_else(|| "Bucket name not provided")?;

    // split the path up to a (bucket, prefix)
    let mut splitn = root_paths.trim_left_matches("s3://").splitn(2, '/');

    // bucket is required, prefix is optional after `/`
    let bucket = splitn.next().unwrap().to_string();
    let prefix = splitn.next().map(|s| s.to_string());

    // create client options
    let client = HttpClient::new()?;
    let region = Region::default();

    // create provided with timeout
    let mut chain = ChainProvider::new();
    chain.set_timeout(Duration::from_millis(500));

    // construct new S3 client
    let s3 = S3Client::new_with(client, chain, region);

    // create our set of metric meters
    let mut chain = metrics::chain(&prefix);

    // iteration token
    let mut token = None;

    loop {
        // create a request to list objects
        let request = ListObjectsV2Request {
            bucket: bucket.clone(),
            prefix: prefix.clone(),
            continuation_token: token,
            ..ListObjectsV2Request::default()
        };

        // execute the request and await the response (blocking)
        let response = s3.list_objects_v2(request).sync()?;

        // check contents (although should always be there)
        if let Some(contents) = response.contents {
            // iterate all objects
            for entry in contents {
                // iterate all metrics meters
                for metric in &mut chain {
                    metric.register(&entry);
                }
            }
        }

        // break if there's no way to continue
        if let None = response.next_continuation_token {
            break;
        }

        // store the token for next iteration
        token = response.next_continuation_token;
    }

    // print all statistics
    for metric in &chain {
        metric.print();
    }

    // done
    Ok(())
}
