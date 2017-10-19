#![cfg(feature = "dax")]

extern crate rusoto_core;
extern crate rusoto_dax;

use rusoto_dax::{DynamodbAccelerator, DynamodbAcceleratorClient, DescribeClustersRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_clusters() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DynamodbAcceleratorClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeClustersRequest::default();

    client.describe_clusters(&request).unwrap();
}
