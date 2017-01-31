#![cfg(feature = "health")]
extern crate rusoto;

use rusoto::health::{HealthClient, DescribeEventsRequest};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_events() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = HealthClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeEventsRequest::default();

    let result = client.describe_events(&request).unwrap();
	println!("{:#?}", result);
}