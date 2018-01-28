#![cfg(feature = "ds")]

extern crate rusoto_core;
extern crate rusoto_ds;

use rusoto_ds::{DirectoryService, DirectoryServiceClient, DescribeTrustsRequest,
                 DescribeDirectoriesRequest};
use rusoto_core::Region;

#[test]
fn should_describe_trusts() {
    let client = DirectoryServiceClient::simple(Region::UsEast1);
    let request = DescribeTrustsRequest::default();

    client.describe_trusts(&request).sync().unwrap();
}

#[test]
fn should_describe_directories() {
    let client = DirectoryServiceClient::simple(Region::UsEast1);
    let request = DescribeDirectoriesRequest::default();

    client.describe_directories(&request).sync().unwrap();
}
