#![cfg(feature = "support")]

extern crate rusoto_core;
extern crate rusoto_support;

use rusoto_support::{AWSSupport, AWSSupportClient, DescribeServicesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_domains() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = AWSSupportClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let request = DescribeServicesRequest::default();

    let result = client.describe_services(&request).unwrap();
    println!("{:#?}", result);
}
