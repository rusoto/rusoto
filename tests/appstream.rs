#![cfg(feature = "appstream")]

extern crate rusoto;

use rusoto::appstream::{AppStreamClient, DescribeFleetsRequest};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_fleets() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = AppStreamClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeFleetsRequest::default();

	let result = client.describe_fleets(&request).unwrap();
	println!("{:#?}", result);
}


