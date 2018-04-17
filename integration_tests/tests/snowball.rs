#![cfg(feature = "snowball")]

extern crate rusoto_core;
extern crate rusoto_snowball;

use rusoto_snowball::{Snowball, SnowballClient, DescribeAddressesRequest};
use rusoto_core::Region;

#[test]
fn should_describe_addresses() {
    let client = SnowballClient::simple(Region::UsEast1);
    let request = DescribeAddressesRequest::default();

    let result = client.describe_addresses(request).sync().unwrap();
    println!("{:#?}", result);
}