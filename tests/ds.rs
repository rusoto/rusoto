#![cfg(feature = "ds")]

extern crate rusoto;

use rusoto::ds::{DirectoryServiceClient, DescribeTrustsRequest, DescribeDirectoriesRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_trusts() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DirectoryServiceClient::new(credentials, Region::UsEast1).unwrap();
    let request = DescribeTrustsRequest::default();

    client.describe_trusts(&request).unwrap();
}

#[test]
fn should_describe_directories() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DirectoryServiceClient::new(credentials, Region::UsEast1).unwrap();
    let request = DescribeDirectoriesRequest::default();

    client.describe_directories(&request).unwrap();
}