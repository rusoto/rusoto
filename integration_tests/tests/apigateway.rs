#![cfg(feature = "apigateway")]

extern crate rusoto_core;
extern crate rusoto_apigateway;

use rusoto_apigateway::{ApiGateway, ApiGatewayClient, GetRestApisRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};


#[test]
fn should_get_rest_apis() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = ApiGatewayClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = GetRestApisRequest::default();

    client.get_rest_apis(&request).unwrap();

}