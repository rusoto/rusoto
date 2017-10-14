#![cfg(feature = "cognito-sync")]

extern crate rusoto_core;
extern crate rusoto_cognito_sync;

use rusoto_cognito_sync::{CognitoSync, CognitoSyncClient, ListIdentityPoolUsageRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_identity_pool_usage() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CognitoSyncClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListIdentityPoolUsageRequest::default();

    let result = client.list_identity_pool_usage(&request).unwrap();
	println!("{:#?}", result);
}