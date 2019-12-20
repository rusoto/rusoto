#![cfg(feature = "ecr")]

extern crate rusoto_core;
extern crate rusoto_ecr;

use rusoto_core::Region;
use rusoto_ecr::{DescribeRepositoriesRequest, Ecr, EcrClient};

#[tokio::test]
async fn should_describe_repositories() {
    let client = EcrClient::new(Region::UsEast1);
    let request = DescribeRepositoriesRequest::default();

    client.describe_repositories(request).await.unwrap();
}
