#![cfg(feature = "dms")]
extern crate rusoto;

use rusoto::dms::{DatabaseMigrationClient, DescribeEndpointsMessage};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_tags() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DatabaseMigrationClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    let request = DescribeEndpointsMessage::default();

    let result = client.describe_endpoints(&request);
	println!("{:#?}", result);
}


