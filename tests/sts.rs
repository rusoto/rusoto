#![cfg(feature = "sts")]

extern crate rusoto;

use rusoto::default_tls_client;
use rusoto::sts::StsClient;
use rusoto::sts::{AssumeRoleRequest, AssumeRoleError};
use rusoto::sts::{GetSessionTokenRequest, GetSessionTokenError};
use rusoto::sts::StsSessionCredentialsProvider;
use rusoto::{DefaultCredentialsProvider, Region, ProvideAwsCredentials};

#[test]
fn main() {
    let credentials = DefaultCredentialsProvider::new().unwrap();

    let sts = StsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    // http://docs.aws.amazon.com/STS/latest/APIReference/Welcome.html
    match sts.assume_role(&AssumeRoleRequest{
            role_arn: "bogus".to_owned(),
            role_session_name: "rusoto_test_session".to_owned(),
            ..Default::default()
        }) {
        Err(AssumeRoleError::Unknown(msg)) =>
            assert!(msg.contains("validation error detected: Value 'bogus' at 'roleArn' failed to satisfy constraint")),
        err =>
            panic!("this should have been an Unknown STS Error: {:?}", err)
    }

    match sts.get_session_token(
        &GetSessionTokenRequest {
            token_code: Some("123456".to_owned()),
            serial_number: Some("123456789".to_owned()),
            ..Default::default()
        }) {
        Err(GetSessionTokenError::Unknown(msg)) =>
            assert!(msg.contains("Please verify your MFA serial number is valid and associated with this user.")),
        err => 
            panic!("this should have been an Unknown STS Error: {:?}", err)
    }

    let sts_creds_provider = StsSessionCredentialsProvider::new(sts, None, None);

    match sts_creds_provider.credentials() {
        Err(e) => panic!("sts credentials provider error: {:?}", e),
        Ok(r) => println!("sts credentials provider result: {:?}", r)
    }
}
