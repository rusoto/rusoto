#![cfg(feature = "cognito-sync")]

extern crate rusoto_core;
extern crate rusoto_cognito_sync;

use rusoto_cognito_sync::{CognitoSync, CognitoSyncClient, ListIdentityPoolUsageRequest};
use rusoto_core::Region;

#[test]
fn should_list_identity_pool_usage() {
    let client = CognitoSyncClient::simple(Region::UsEast1);
    let request = ListIdentityPoolUsageRequest::default();

    let result = client.list_identity_pool_usage(&request).sync().unwrap();
    println!("{:#?}", result);
}