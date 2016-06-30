#![cfg(feature = "cloudtrail")]

extern crate rusoto;

use rusoto::cloudtrail::{CloudTrailClient, DescribeTrailsRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_trails() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudTrailClient::new(credentials, Region::UsEast1);
    let request = DescribeTrailsRequest::default();

    client.describe_trails(&request).unwrap();
}