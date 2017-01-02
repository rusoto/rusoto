#![cfg(feature = "autoscaling")]

extern crate rusoto;

use rusoto::autoscaling::{AutoscalingClient, AutoScalingGroupNamesType};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_auto_scaling_groups() {
    let client = AutoscalingClient::new(DefaultCredentialsProvider::new().unwrap(), Region::UsEast1);
    let request = AutoScalingGroupNamesType::default();

 	let response = client.describe_auto_scaling_groups(&request).unwrap();
    println!("{:#?}", response);
}

