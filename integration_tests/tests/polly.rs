#![cfg(feature = "polly")]

extern crate rusoto_core;
extern crate rusoto_polly;

use rusoto_core::Region;
use rusoto_polly::{DescribeVoicesInput, Polly, PollyClient};

#[tokio::test]
async fn should_describe_voices() {
    let client = PollyClient::new(Region::UsEast1);
    let request = DescribeVoicesInput::default();

    println!("{:?}", client.describe_voices(request).await.unwrap());
}
