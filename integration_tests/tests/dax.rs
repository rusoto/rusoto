#![cfg(feature = "dax")]

extern crate rusoto_core;
extern crate rusoto_dax;

use rusoto_dax::{DynamodbAccelerator, DynamodbAcceleratorClient, DescribeClustersRequest};
use rusoto_core::Region;

#[test]
fn should_describe_clusters() {
    let client = DynamodbAcceleratorClient::new(Region::UsEast1);
    let request = DescribeClustersRequest::default();

    client.describe_clusters(request).sync().unwrap();
}
