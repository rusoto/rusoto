#![cfg(feature = "iam")]

extern crate rusoto;

use rusoto::iam::IamClient;
use rusoto::iam::{GetUserRequest, ListUsersRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn get_user() {
    let credentials = DefaultCredentialsProvider::new().unwrap();

    let iam = IamClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    // http://docs.aws.amazon.com/IAM/latest/APIReference/Welcome.html
    let request = GetUserRequest {
        ..Default::default()
    };
    iam.get_user(&request).unwrap();
}

#[test]
fn list_users() {
    let credentials = DefaultCredentialsProvider::new().unwrap();

    let iam = IamClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    // http://docs.aws.amazon.com/IAM/latest/APIReference/Welcome.html
    let request = ListUsersRequest {
        ..Default::default()
    };
    iam.list_users(&request).unwrap();
}
