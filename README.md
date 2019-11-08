# S3 Meta
[![Crates.io](https://img.shields.io/crates/v/s3-meta.svg)](https://crates.io/crates/s3-meta) [![Build Status](https://img.shields.io/travis/whitfin/s3-meta.svg)](https://travis-ci.org/whitfin/s3-meta)

**This tool has been migrated into [s3-utils](https://github.com/whitfin/s3-utils), please use that crate for future updates.**

A simple tool to gather data about an S3 bucket (or subsection thereof). Designed to be simple, and (sort-of) shell consumable. Metadata will be added as it becomes available, and S3 interaction is controlled by [rusoto_s3](https://crates.io/crates/rusoto_s3).

## Installation

You can install `s3-meta` from either this repository, or from Crates (once it's published):

```shell
# install from Cargo
$ cargo install s3-meta

# install the latest from GitHub
$ cargo install --git https://github.com/whitfin/s3-meta.git
```

## Usage

Credentials can be configured by following the instructions on the [AWS Documentation](https://docs.aws.amazon.com/cli/latest/userguide/cli-environment.html), although examples will use environment variables for the sake of clarity.

You can retrieve metadata about a bucket using the following form (making sure your region matches your bucket AWS region):

```shell
$ AWS_ACCESS_KEY_ID=MY_ACCESS_KEY_ID \
    AWS_SECRET_ACCESS_KEY=MY_SECRET_ACCESS_KEY \
    AWS_DEFAULT_REGION=us-west-2 \
    s3-meta my.bucket.name
```

If you want to retrieve metadata about a "subdirectory" of a bucket, you can provide a prefix using this format:

```shell
$ AWS_ACCESS_KEY_ID=MY_ACCESS_KEY_ID \
    AWS_SECRET_ACCESS_KEY=MY_SECRET_ACCESS_KEY \
    AWS_DEFAULT_REGION=us-west-2 \
    s3-meta my.bucket.name/my/directory/path
```

Don't forget to add a space to the start of your command if you're going to inline your credentials as above!

## Output

Output is pretty straightforward, and follows a relatively simple format which is easily extensible, and hopefully convenient in shell pipelines. There may be changes made to this format to make it easier to consume (spaces placed to make splitting easier, unformatted numbers, etc).

Below is an example based on a real S3 bucket (although the file names have been tweaked :)):

```
[general]
total_time=7s
total_space=1.94TB
total_files=51,152

[file_size]
average_file_size=37.95MB
average_file_bytes=37949529
largest_file_size=1.82GB
largest_file_bytes=1818900684
largest_file_name=path/to/my_largest_file.txt.gz
smallest_file_size=54B
smallest_file_bytes=54
smallest_file_name=path/to/my_smallest_file.txt.gz
smallest_file_others=12

[extensions]
unique_extensions=1
most_frequent_extension=gz

[modification]
earliest_file_date=2016-06-11T17:36:57.000Z
earliest_file_name=path/to/my_earliest_file.txt.gz
earliest_file_others=3
latest_file_date=2017-01-01T00:03:19.000Z
latest_file_name=path/to/my_latest_file.txt.gz
```

This sample is based on the initial builds of `s3-meta`. Depending on when you come to this tool, there may be more (or less) included in the output above.
