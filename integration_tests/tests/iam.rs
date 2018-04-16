#![cfg(feature = "iam")]

extern crate rusoto_core;
extern crate rusoto_iam;

use rusoto_iam::{Iam, IamClient};
use rusoto_iam::{GetUserRequest, ListUsersRequest};
use rusoto_core::Region;

#[test]
fn get_user() {
    let iam = IamClient::simple(Region::UsEast1);

    // http://docs.aws.amazon.com/IAM/latest/APIReference/Welcome.html
    let request = GetUserRequest { ..Default::default() };
    iam.get_user(request).sync().unwrap();
}

#[test]
fn list_users() {
    let iam = IamClient::simple(Region::UsEast1);

    // http://docs.aws.amazon.com/IAM/latest/APIReference/Welcome.html
    let request = ListUsersRequest { ..Default::default() };
    iam.list_users(request).sync().unwrap();
}
