extern crate humantime;
extern crate pretty_bytes;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_s3;
extern crate tokio_core;

use rusoto_core::{reactor::RequestDispatcher, region::Region};
use rusoto_credential::ChainProvider;
use rusoto_s3::{ListObjectsV2Request, S3, S3Client};
use std::time::{Duration, SystemTime};
use tokio_core::reactor::Core;

mod bounded;
mod util;

use bounded::Bounded;

fn main() {
    // actual start time of execution
    let start_time = SystemTime::now();

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

    // bounds for tracking modification timestamps
    let mut latest_file = Bounded::new("".to_string());
    let mut earliest_file = Bounded::new("".to_string());

    // bounds for tracking file sizes
    let mut largest_file = Bounded::new(0_u64);
    let mut smallest_file = Bounded::new(0_u64);

    // simple counter tracking
    let mut total_keys = 0_u64;
    let mut total_size = 0_u64;

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

        // increment the count of keys retrieved
        total_keys += response.key_count.unwrap() as u64;

        // check contents (although should always be there)
        if let Some(contents) = response.contents {
            // iterate all objects
            for entry in contents {
                // pull various metadata
                let key = entry.key.unwrap();
                let size = entry.size.unwrap() as u64;
                let modified = entry.last_modified.unwrap();

                // apply any bounded updates for the file size and modified stamp
                bounded::apply(&mut earliest_file, &mut latest_file, &key, &modified);
                bounded::apply(&mut smallest_file, &mut largest_file, &key, &size);

                // increment the total size
                total_size += size as u64;
            }
        }

        // break if there's no way to continue
        if let None = response.next_continuation_token {
            break;
        }

        // store the token for next iteration
        token = response.next_continuation_token;
    }

    // get average file size, protect against /0
    let average_file = match total_keys {
        0 => 0,
        v => total_size / v,
    };

    // task done, so check execution time
    let task_duration = Duration::from_secs(
        SystemTime::now()
            .duration_since(start_time)
            .expect("SystemTime::duration_since failed")
            .as_secs(),
    );

    // first segment: general
    println!("[general]");
    util::log_pair("total_time", humantime::format_duration(task_duration));
    util::log_pair("total_space", util::convert_bytes(total_size));
    util::log_pair("total_files", util::comma(total_keys));

    // next segment: file_size
    util::log_head("file_size");

    // log the average size as both readable and bytes
    util::log_pair("average_file_size", util::convert_bytes(average_file));
    util::log_pair("average_file_bytes", average_file);

    // log out the bounds of the largest file
    util::log_bound("largest_file", largest_file, |size| {
        util::log_pair("largest_file_size", util::convert_bytes(size));
        util::log_pair("largest_file_bytes", size);
    });

    // log out the bounds of the smallest file
    util::log_bound("smallest_file", smallest_file, |size| {
        util::log_pair("smallest_file_size", util::convert_bytes(size));
        util::log_pair("smallest_file_bytes", size);
    });

    // next segment: modification
    util::log_head("modification");

    // log out the bounds of the earliest file
    util::log_bound("earliest_file", earliest_file, |date| {
        util::log_pair("earliest_file_date", date);
    });

    // log out the bounds of the latest file
    util::log_bound("latest_file", latest_file, |date| {
        util::log_pair("latest_file_date", date);
    });
}
