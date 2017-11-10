#![cfg(feature = "polly")]

extern crate rusoto_core;
extern crate rusoto_polly;

use rusoto_polly::{Polly, PollyClient, DescribeVoicesInput};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_voices() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = PollyClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeVoicesInput::default();

    println!("{:?}", client.describe_voices(&request).sync().unwrap());
}
