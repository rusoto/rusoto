#![cfg(feature = "iam")]

extern crate rusoto_core;
extern crate rusoto_iam;

use rusoto_core::Region;
use rusoto_iam::{GetUserRequest, ListUsersRequest};
use rusoto_iam::{Iam, IamClient};

#[tokio::test]
async fn get_user() {
    let iam = IamClient::new(Region::UsEast1);

    // http://docs.aws.amazon.com/IAM/latest/APIReference/Welcome.html
    let request = GetUserRequest {
        ..Default::default()
    };
    iam.get_user(request).await.unwrap();
}

#[tokio::test]
async fn list_users() {
    let iam = IamClient::new(Region::UsEast1);

    // http://docs.aws.amazon.com/IAM/latest/APIReference/Welcome.html
    let request = ListUsersRequest {
        ..Default::default()
    };
    iam.list_users(request).await.unwrap();
}
