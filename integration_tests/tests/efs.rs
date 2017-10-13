#![cfg(feature = "efs")]

extern crate rusoto_core;
extern crate rusoto_efs;

use rusoto_efs::{Efs, EfsClient, DescribeFileSystemsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_filesystems() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = EfsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeFileSystemsRequest::default();

    let result = client.describe_file_systems(&request).unwrap();
    println!("{:#?}", result);
}
