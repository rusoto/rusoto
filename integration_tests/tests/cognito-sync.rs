#![cfg(feature = "cognito-sync")]

extern crate rusoto_cognito_sync;
extern crate rusoto_core;

use rusoto_cognito_sync::{CognitoSync, CognitoSyncClient, ListIdentityPoolUsageRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_list_identity_pool_usage() {
    let client = CognitoSyncClient::new(Region::UsEast1);
    let request = ListIdentityPoolUsageRequest::default();

    let result = client.list_identity_pool_usage(request).await.unwrap();
    println!("{:#?}", result);
}
