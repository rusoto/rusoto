#![cfg(feature = "polly")]

extern crate rusoto_core;
extern crate rusoto_polly;

use rusoto_polly::{Polly, PollyClient, DescribeVoicesInput};
use rusoto_core::Region;

#[test]
fn should_describe_voices() {
    let client = PollyClient::simple(Region::UsEast1);
    let request = DescribeVoicesInput::default();

    println!("{:?}", client.describe_voices(request).sync().unwrap());
}
