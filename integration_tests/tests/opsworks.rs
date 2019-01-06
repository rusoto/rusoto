#![cfg(feature = "opsworks")]

extern crate rusoto_core;
extern crate rusoto_opsworks;

use rusoto_core::Region;
use rusoto_opsworks::{DescribeStacksRequest, OpsWorks, OpsWorksClient};

#[test]
fn should_describe_stacks() {
    let client = OpsWorksClient::new(Region::UsEast1);
    let request = DescribeStacksRequest::default();

    client.describe_stacks(request).sync().unwrap();
}

#[test]
fn should_describe_my_user_profile() {
    let client = OpsWorksClient::new(Region::UsEast1);

    client.describe_my_user_profile().sync().unwrap();
}
