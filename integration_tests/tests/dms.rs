#![cfg(feature = "dms")]

extern crate rusoto_core;
extern crate rusoto_dms;

use rusoto_dms::{DatabaseMigrationService, DatabaseMigrationServiceClient, DescribeEndpointsMessage};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_tags() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DatabaseMigrationServiceClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeEndpointsMessage::default();

    let result = client.describe_endpoints(&request);
	println!("{:#?}", result);
}