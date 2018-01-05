#![cfg(feature = "cognito-idp")]

extern crate rusoto_core;
extern crate rusoto_cognito_idp;

use rusoto_cognito_idp::{CognitoIdentityProvider, CognitoIdentityProviderClient, ListUserPoolsRequest};
use rusoto_core::Region;

#[test]
fn should_list_user_pools() {
    let client = CognitoIdentityProviderClient::simple(Region::UsEast1);
    let request = ListUserPoolsRequest{
    	max_results: 10,
    	..Default::default()
    };

    let result = client.list_user_pools(&request).sync().unwrap();
	println!("{:#?}", result);
}