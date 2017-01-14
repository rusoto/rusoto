#![cfg(feature = "cognito-identity")]

extern crate rusoto;

use rusoto::cognitoidentity::{CognitoIdentityClient, ListIdentitiesInput, ListIdentitiesError, ListIdentityPoolsInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_identity_pools() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CognitoIdentityClient::new(credentials, Region::UsEast1).unwrap();

    let mut request = ListIdentityPoolsInput::default();
    request.max_results = 10;

    client.list_identity_pools(&request).unwrap();
}

#[test]
fn should_handle_validation_errors_gracefully() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CognitoIdentityClient::new(credentials, Region::UsEast1).unwrap();

    let mut request = ListIdentitiesInput::default();
    request.max_results = 10;
    request.identity_pool_id = "invalid".to_string();

    match client.list_identities(&request) {
    	Err(ListIdentitiesError::Validation(msg)) => {
    		assert!(msg.contains("identityPoolId"))
    	},
    	err @ _ => panic!("Expected Validation error - got {:#?}", err),
    };
}
