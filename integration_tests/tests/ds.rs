#![cfg(feature = "ds")]

extern crate rusoto;

use rusoto::ds::{DirectoryService, DirectoryServiceClient, DescribeTrustsRequest,
                 DescribeDirectoriesRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_describe_trusts() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        DirectoryServiceClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeTrustsRequest::default();

    client.describe_trusts(&request).unwrap();
}

#[test]
fn should_describe_directories() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        DirectoryServiceClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeDirectoriesRequest::default();

    client.describe_directories(&request).unwrap();
}
