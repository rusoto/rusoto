#![cfg(feature = "opsworkscm")]

extern crate rusoto_core;
extern crate rusoto_opsworkscm;

use rusoto_core::Region;
use rusoto_opsworkscm::{DescribeServersRequest, OpsWorksCM, OpsWorksCMClient};

#[test]
fn should_describe_servers() {
    let client = OpsWorksCMClient::new(Region::UsEast1);
    let request = DescribeServersRequest::default();

    let result = client.describe_servers(request).sync().unwrap();
    println!("{:#?}", result);
}
