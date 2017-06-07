#![cfg(feature = "ecr")]

extern crate rusoto_core;
extern crate rusoto_ecr;

use rusoto_ecr::{Ecr, EcrClient, DescribeRepositoriesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_repositories() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = EcrClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeRepositoriesRequest::default();

    client.describe_repositories(&request).unwrap();
}
