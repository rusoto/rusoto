#![cfg(feature = "application-autoscaling")]

extern crate rusoto_core;
extern crate rusoto_application_autoscaling;

use rusoto_application_autoscaling::{ApplicationAutoScaling, ApplicationAutoScalingClient, DescribeScalingPoliciesRequest};
use rusoto_core::Region;

#[test]
fn should_describe_scaling_policies() {
    let client = ApplicationAutoScalingClient::new(Region::UsEast1);

    let request = DescribeScalingPoliciesRequest{
        service_namespace: "ec2".to_owned(),
        ..Default::default()
    };

    client.describe_scaling_policies(request).sync().unwrap();
}
