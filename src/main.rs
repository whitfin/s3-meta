extern crate humantime;
extern crate pretty_bytes;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_s3;
extern crate tokio_core;

use rusoto_core::{reactor::RequestDispatcher, region::Region};
use rusoto_credential::ChainProvider;
use rusoto_s3::{ListObjectsV2Request, S3, S3Client};
use tokio_core::reactor::Core;

mod bounded;
mod metrics;
mod util;

fn main() {
    // grab the root path of the S3 location to use and split
    let root_paths = std::env::args().skip(1).next().unwrap();
    let mut splitn = root_paths.splitn(2, '/');

    // bucket is required, prefix is optional after `/`
    let bucket = splitn.next().unwrap().to_string();
    let prefix = splitn.next().map(|s| s.to_string());

    // asynchronous core for rusoto
    let core = Core::new().unwrap();

    // construct new S3 client
    let s3 = S3Client::new(
        RequestDispatcher::default(),
        ChainProvider::new(&core.handle()),
        Region::default(),
    );

    // create our set of metric meters
    let mut chain = metrics::chain();

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
        let response = s3.list_objects_v2(&request).sync().unwrap();

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
}
