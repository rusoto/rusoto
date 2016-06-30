#![cfg(feature = "opsworks")]

extern crate rusoto;

use rusoto::opsworks::{OpsWorksClient, DescribeStacksRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_stacks() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = OpsWorksClient::new(credentials, Region::UsEast1);
    let request = DescribeStacksRequest::default();

    client.describe_stacks(&request).unwrap();
}

#[test]
fn should_describe_my_user_profile() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = OpsWorksClient::new(credentials, Region::UsEast1);

    client.describe_my_user_profile().unwrap();
}