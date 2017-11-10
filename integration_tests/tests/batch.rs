#![cfg(feature = "batch")]

extern crate rusoto_core;
extern crate rusoto_batch;

use rusoto_batch::{Batch, BatchClient, DescribeJobDefinitionsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};


#[test]
fn should_get_rest_apis() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = BatchClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeJobDefinitionsRequest::default();

    client.describe_job_definitions(&request).sync().unwrap();

}