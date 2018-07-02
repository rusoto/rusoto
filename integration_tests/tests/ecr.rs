#![cfg(feature = "ecr")]

extern crate rusoto_core;
extern crate rusoto_ecr;

use rusoto_ecr::{Ecr, EcrClient, DescribeRepositoriesRequest};
use rusoto_core::Region;

#[test]
fn should_describe_repositories() {
    let client = EcrClient::new(Region::UsEast1);
    let request = DescribeRepositoriesRequest::default();

    client.describe_repositories(request).sync().unwrap();
}
