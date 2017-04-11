#![cfg(feature = "opsworkscm")]
extern crate rusoto;

use rusoto::opsworkscm::{OpsWorksCmClient, DescribeServersRequest};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_describe_servers() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = OpsWorksCmClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeServersRequest::default();

    let result = client.describe_servers(&request).unwrap();
	println!("{:#?}", result);
}