#![cfg(feature = "cognito-identity")]

extern crate rusoto_core;
extern crate rusoto_cognito_identity;

use rusoto_cognito_identity::{CognitoIdentity, CognitoIdentityClient, ListIdentitiesInput,
                              ListIdentitiesError, ListIdentityPoolsInput};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_identity_pools() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        CognitoIdentityClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let mut request = ListIdentityPoolsInput::default();
    request.max_results = 10;

    client.list_identity_pools(&request).unwrap();
}

#[test]
fn should_handle_validation_errors_gracefully() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        CognitoIdentityClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let mut request = ListIdentitiesInput::default();
    request.max_results = 10;
    request.identity_pool_id = "invalid".to_string();

    match client.list_identities(&request) {
        Err(ListIdentitiesError::Validation(msg)) => assert!(msg.contains("identityPoolId")),
        err @ _ => panic!("Expected Validation error - got {:#?}", err),
    };
}
