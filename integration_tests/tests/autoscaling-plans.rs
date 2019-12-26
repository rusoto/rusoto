#![cfg(feature = "autoscaling-plans")]

extern crate rusoto_autoscaling_plans;
extern crate rusoto_core;

use rusoto_autoscaling_plans::{
    AutoscalingPlans, AutoscalingPlansClient, DescribeScalingPlansRequest,
};
use rusoto_core::Region;

#[tokio::test]
async fn should_describe_scaling_plans() {
    let client = AutoscalingPlansClient::new(Region::UsEast1);

    let request = DescribeScalingPlansRequest::default();

    let res = client.describe_scaling_plans(request).await;

    match res {
        Err(e) => panic!("Error getting scaling plans: {:?}", e),
        Ok(response) => println!("Got this response: {:?}", response),
    }
}
