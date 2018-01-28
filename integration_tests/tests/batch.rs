#![cfg(feature = "batch")]

extern crate rusoto_core;
extern crate rusoto_batch;

use rusoto_batch::{Batch, BatchClient, DescribeJobDefinitionsRequest};
use rusoto_core::Region;

#[test]
fn should_get_rest_apis() {
    let client = BatchClient::simple(Region::UsEast1);
    let request = DescribeJobDefinitionsRequest::default();

    client.describe_job_definitions(&request).sync().unwrap();

}