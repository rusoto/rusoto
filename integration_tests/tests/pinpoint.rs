#![cfg(feature = "pinpoint")]

extern crate rusoto_core;
extern crate rusoto_pinpoint;

use rusoto_polly::{Pinpoint, PinpointClient, DescribeAppsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_apps() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = PinpointClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeAppsRequest::default();

    let result = client.describe_apps(&request).unwrap();
    println!("Result: {:?}", result);
}
