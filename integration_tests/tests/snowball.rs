#![cfg(feature = "snowball")]

extern crate rusoto_core;
extern crate rusoto_snowball;

use rusoto_core::Region;
use rusoto_snowball::{DescribeAddressesRequest, Snowball, SnowballClient};

#[test]
fn should_describe_addresses() {
    let client = SnowballClient::new(Region::UsEast1);
    let request = DescribeAddressesRequest::default();

    let result = client.describe_addresses(request).sync().unwrap();
    println!("{:#?}", result);
}
