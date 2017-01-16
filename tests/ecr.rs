#![cfg(feature = "ecr")]

extern crate rusoto;

use rusoto::ecr::{EcrClient, DescribeRepositoriesRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_describe_repositories() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = EcrClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeRepositoriesRequest::default();

    client.describe_repositories(&request).unwrap();
}

