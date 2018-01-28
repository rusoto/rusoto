#![cfg(feature = "efs")]

extern crate rusoto_core;
extern crate rusoto_efs;

use rusoto_efs::{Efs, EfsClient, DescribeFileSystemsRequest};
use rusoto_core::Region;

#[test]
fn should_describe_filesystems() {
    let client = EfsClient::simple(Region::UsEast1);
    let request = DescribeFileSystemsRequest::default();

    let result = client.describe_file_systems(&request).sync().unwrap();
    println!("{:#?}", result);
}
