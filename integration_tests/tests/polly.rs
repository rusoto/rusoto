#![cfg(feature = "polly")]

extern crate rusoto_core;
extern crate rusoto_polly;

use rusoto_core::Region;
use rusoto_polly::{DescribeVoicesRequest, Polly, PollyClient};

#[test]
fn should_describe_voices() {
    let client = PollyClient::new(Region::UsEast1);
    let request = DescribeVoicesRequest::default();

    println!("{:?}", client.describe_voices(request).sync().unwrap());
}
