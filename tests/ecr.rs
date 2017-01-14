#![cfg(feature = "ecr")]

extern crate rusoto;

use rusoto::ecr::{EcrClient, DescribeRepositoriesRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_repositories() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = EcrClient::new(credentials, Region::UsEast1).unwrap();
    let request = DescribeRepositoriesRequest::default();

    client.describe_repositories(&request).unwrap();
}

