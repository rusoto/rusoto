#![cfg(feature = "sts")]

extern crate futures;
extern crate rusoto_core;
extern crate rusoto_sts;
extern crate rusoto_ec2;

use futures::Future;

use rusoto_sts::{Sts, StsClient};
use rusoto_sts::{AssumeRoleRequest, AssumeRoleError};
use rusoto_sts::{GetSessionTokenRequest, GetSessionTokenError};
use rusoto_sts::{StsSessionCredentialsProvider, StsAssumeRoleSessionCredentialsProvider};
use rusoto_core::{ProvideAwsCredentials, Region};
use rusoto_core::request::HttpClient;
use rusoto_ec2::Ec2Client;

#[test]
fn main() {
    let sts = StsClient::new(Region::UsEast1);

    // http://docs.aws.amazon.com/STS/latest/APIReference/Welcome.html
    match sts.assume_role(AssumeRoleRequest{
            role_arn: "bogus".to_owned(),
            role_session_name: "rusoto_test_session".to_owned(),
            ..Default::default()
        }).sync() {
        Err(AssumeRoleError::Unknown(msg)) =>
            assert!(msg.contains("validation error detected: Value 'bogus' at 'roleArn' failed to satisfy constraint")),
        err =>
            panic!("this should have been an Unknown STS Error: {:?}", err)
    }

    match sts.get_session_token(
        GetSessionTokenRequest {
            token_code: Some("123456".to_owned()),
            serial_number: Some("123456789".to_owned()),
            ..Default::default()
        }).sync() {
        Err(GetSessionTokenError::Unknown(msg)) =>
            assert!(msg.contains("Please verify your MFA serial number is valid and associated with this user.")),
        err => 
            panic!("this should have been an Unknown STS Error: {:?}", err)
    }

    let sts_creds_provider = StsSessionCredentialsProvider::new(sts, None, None);

    match sts_creds_provider.credentials().wait() {
        Err(e) => panic!("sts credentials provider error: {:?}", e),
        Ok(r) => println!("sts credentials provider result: {:?}", r)
    }

    // check that it's possible to create a new ec2 client with sts
    let provider = StsAssumeRoleSessionCredentialsProvider::new(
        StsClient::new(Region::UsEast1),
        "arn:aws:sts::1122334455:role/myrole".to_owned(),
        "session-name".to_owned(),
        None,
        None,
        None,
        None,
    );
    Ec2Client::new_with(HttpClient::new().unwrap(), provider, Region::default());
}
