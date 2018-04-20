#![cfg(feature = "opsworkscm")]

extern crate rusoto_core;
extern crate rusoto_opsworkscm;

use rusoto_opsworkscm::{OpsWorksCM, OpsWorksCMClient, DescribeServersRequest};
use rusoto_core::Region;

#[test]
fn should_describe_servers() {
    let client = OpsWorksCMClient::simple(Region::UsEast1);
    let request = DescribeServersRequest::default();

    let result = client.describe_servers(request).sync().unwrap();
	println!("{:#?}", result);
}