#![cfg(feature = "cloudtrail")]

extern crate rusoto_cloudtrail;
extern crate rusoto_core;

use rusoto_cloudtrail::{CloudTrail, CloudTrailClient, DescribeTrailsRequest};
use rusoto_core::Region;

#[test]
fn should_describe_trails() {
    let client = CloudTrailClient::new(Region::UsEast1);
    let request = DescribeTrailsRequest::default();

    client.describe_trails(request).sync().unwrap();
}
