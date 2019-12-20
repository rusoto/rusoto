#![cfg(feature = "batch")]

extern crate rusoto_batch;
extern crate rusoto_core;

use rusoto_batch::{Batch, BatchClient, DescribeJobDefinitionsRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_get_rest_apis() {
    let client = BatchClient::new(Region::UsEast1);
    let request = DescribeJobDefinitionsRequest::default();

    client.describe_job_definitions(request).await.unwrap();
}
