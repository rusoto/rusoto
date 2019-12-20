#![cfg(feature = "dax")]

extern crate rusoto_core;
extern crate rusoto_dax;

use rusoto_core::Region;
use rusoto_dax::{DescribeClustersRequest, DynamodbAccelerator, DynamodbAcceleratorClient};

#[tokio::test]
async fn should_describe_clusters() {
    let client = DynamodbAcceleratorClient::new(Region::UsEast1);
    let request = DescribeClustersRequest::default();

    client.describe_clusters(request).await.unwrap();
}
