#![cfg(feature = "autoscaling-plans")]

extern crate rusoto_core;
extern crate rusoto_autoscaling_plans;

use rusoto_autoscaling_plans::{AutoscalingPlans, AutoscalingPlansClient, DescribeScalingPlansRequest};
use rusoto_core::Region;

#[test]
fn should_describe_scaling_plans() {
    let client = AutoscalingPlansClient::new(Region::UsEast1);

    let request = DescribeScalingPlansRequest::default();

    let res = client.describe_scaling_plans(request).sync();

    match res {
        Err(e) => panic!("Error getting scaling plans: {:?}", e),
        Ok(response) => println!("Got this response: {:?}", response),
    }
}
