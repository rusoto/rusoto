#![cfg(feature = "ec2")]

extern crate rusoto;

use rusoto::ec2::{Ec2Client, DescribeInstancesRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use std::error::Error;

#[test]
fn main() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let ec2 = Ec2Client::new(credentials, Region::UsEast1);

    let mut req = DescribeInstancesRequest::default();
    req.instance_ids = Some(vec!["i-00000000".into(), "i-00000001".into()]);
    match ec2.describe_instances(&req) {
        Ok(_) => {
            panic!("DescribeInstances should fail");
        },
        Err(error) => {
            assert!(error.description().contains("<Message>The instance IDs 'i-00000000, i-00000001' do not exist</Message>"), "Missing error message");
        }
    }

}
