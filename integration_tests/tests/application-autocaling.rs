#![cfg(feature = "application-autoscaling")]

extern crate rusoto_core;
extern crate rusoto_application_autoscaling;

use rusoto_application_autoscaling::{ApplicationAutoScaling, ApplicationAutoScalingClient, DescribeScalingPoliciesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_scaling_policies() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = ApplicationAutoScalingClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let request = DescribeScalingPoliciesRequest{
        service_namespace: "ec2".to_owned(),
        ..Default::default()
    };

    client.describe_scaling_policies(&request).sync().unwrap();
}
