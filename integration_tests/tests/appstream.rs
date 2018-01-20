#![cfg(feature = "appstream")]

extern crate rusoto_core;
extern crate rusoto_appstream;
extern crate env_logger;

use rusoto_appstream::{AppStream, AppStreamClient, DescribeFleetsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_fleets() {
    let _ = env_logger::try_init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = AppStreamClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeFleetsRequest::default();

	let result = client.describe_fleets(&request).unwrap();
	println!("{:#?}", result);
}