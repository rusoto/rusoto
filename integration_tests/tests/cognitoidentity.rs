#![cfg(feature = "cognito-identity")]

extern crate rusoto_cognito_identity;
extern crate rusoto_core;

use rusoto_cognito_identity::{
    CognitoIdentity, CognitoIdentityClient, ListIdentitiesRequest,
    ListIdentityPoolsRequest,
};
use rusoto_core::{Region, RusotoError};

#[test]
fn should_list_identity_pools() {
    let client = CognitoIdentityClient::new(Region::UsEast1);

    let mut request = ListIdentityPoolsRequest::default();
    request.max_results = 10;

    client.list_identity_pools(request).sync().unwrap();
}

#[test]
fn should_handle_validation_errors_gracefully() {
    let client = CognitoIdentityClient::new(Region::UsEast1);

    let mut request = ListIdentitiesRequest::default();
    request.max_results = 10;
    request.identity_pool_id = "invalid".to_string();

    match client.list_identities(request).sync() {
        Err(RusotoError::Validation(msg)) => assert!(msg.contains("identityPoolId")),
        err @ _ => panic!("Expected Validation error - got {:#?}", err),
    };
}
