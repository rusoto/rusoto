#![cfg(feature = "cognito-idp")]

extern crate rusoto_cognito_idp;
extern crate rusoto_core;

use rusoto_cognito_idp::{
    CognitoIdentityProvider, CognitoIdentityProviderClient, ListUserPoolsRequest,
};
use rusoto_core::Region;

#[tokio::test]
async fn should_list_user_pools() {
    let client = CognitoIdentityProviderClient::new(Region::UsEast1);
    let request = ListUserPoolsRequest {
        max_results: 10,
        ..Default::default()
    };

    let result = client.list_user_pools(request).await.unwrap();
    println!("{:#?}", result);
}
