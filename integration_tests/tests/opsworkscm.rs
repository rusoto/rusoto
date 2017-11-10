#![cfg(feature = "opsworkscm")]

extern crate rusoto_core;
extern crate rusoto_opsworkscm;

use rusoto_opsworkscm::{OpsWorksCM, OpsWorksCMClient, DescribeServersRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_servers() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = OpsWorksCMClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeServersRequest::default();

    let result = client.describe_servers(&request).sync().unwrap();
	println!("{:#?}", result);
}