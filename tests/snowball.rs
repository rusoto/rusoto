#![cfg(feature = "snowball")]

extern crate rusoto;

use rusoto::snowball::{SnowballClient, DescribeAddressesRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_describe_addresses() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SnowballClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeAddressesRequest::default();

    let result = client.describe_addresses(&request).unwrap();
    println!("{:#?}", result);
}


