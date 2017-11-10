#![cfg(feature = "snowball")]

extern crate rusoto_core;
extern crate rusoto_snowball;

use rusoto_snowball::{Snowball, SnowballClient, DescribeAddressesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_addresses() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SnowballClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeAddressesRequest::default();

    let result = client.describe_addresses(&request).sync().unwrap();
    println!("{:#?}", result);
}