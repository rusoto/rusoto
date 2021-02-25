#![cfg(feature = "application-autoscaling")]

extern crate rusoto_application_autoscaling;
extern crate rusoto_core;

use rusoto_application_autoscaling::{
    ApplicationAutoScaling, ApplicationAutoScalingClient, DescribeScalingPoliciesRequest,
    ServiceNamespace,
};
use rusoto_core::Region;

#[tokio::test]
async fn should_describe_scaling_policies() {
    let client = ApplicationAutoScalingClient::new(Region::UsEast1);

    let request = DescribeScalingPoliciesRequest {
        service_namespace: ServiceNamespace::Ec2,
        ..Default::default()
    };

    client.describe_scaling_policies(request).await.unwrap();
}
