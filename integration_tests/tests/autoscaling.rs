#![cfg(feature = "autoscaling")]

extern crate rusoto_autoscaling;
extern crate rusoto_core;

use rusoto_autoscaling::{DescribeAutoScalingGroupsRequest, Autoscaling, AutoscalingClient};
use rusoto_core::Region;

#[test]
fn should_describe_auto_scaling_groups() {
    let client = AutoscalingClient::new(Region::UsEast1);
    let request = DescribeAutoScalingGroupsRequest::default();

    let response = client.describe_auto_scaling_groups(request).sync().unwrap();
    println!("{:#?}", response);
}
