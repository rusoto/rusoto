#![cfg(feature = "sts")]

use rusoto_core::request::HttpClient;
use rusoto_core::{credential::ProvideAwsCredentials, Region, RusotoError};
use rusoto_ec2::Ec2Client;
use rusoto_sts::{AssumeRoleRequest, GetSessionTokenRequest};
use rusoto_sts::{Sts, StsClient};
use rusoto_sts::{StsAssumeRoleSessionCredentialsProvider, StsSessionCredentialsProvider};

#[tokio::test]
async fn main() {
    let sts = StsClient::new(Region::UsEast1);

    // http://docs.aws.amazon.com/STS/latest/APIReference/Welcome.html
    let assume_role_res = sts
        .assume_role(AssumeRoleRequest {
            role_arn: "bogus".to_owned(),
            role_session_name: "rusoto_test_session".to_owned(),
            ..Default::default()
        })
        .await;
    match assume_role_res {
        Err(RusotoError::Unknown(http_res)) => {
            let msg = ::std::str::from_utf8(&http_res.body).unwrap();
            assert!(msg.contains("validation error detected: Value 'bogus' at 'roleArn' failed to satisfy constraint"))
        }
        _ => panic!(
            "this should have been an Unknown STS Error: {:?}",
            assume_role_res
        ),
    }

    let get_session_token_res = sts
        .get_session_token(GetSessionTokenRequest {
            token_code: Some("123456".to_owned()),
            serial_number: Some("123456789".to_owned()),
            ..Default::default()
        })
        .await;
    match get_session_token_res {
        Err(RusotoError::Unknown(http_res)) => {
            let msg = ::std::str::from_utf8(&http_res.body).unwrap();
            assert!(msg.contains(
                "Please verify your MFA serial number is valid and associated with this user."
            ))
        }
        _ => panic!(
            "this should have been an Unknown STS Error: {:?}",
            get_session_token_res
        ),
    }

    let sts_creds_provider = StsSessionCredentialsProvider::new(sts, None, None);

    match sts_creds_provider.credentials().await {
        Err(e) => panic!("sts credentials provider error: {:?}", e),
        Ok(r) => println!("sts credentials provider result: {:?}", r),
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
