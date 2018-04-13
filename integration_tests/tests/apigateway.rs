#![cfg(feature = "apigateway")]

extern crate rusoto_core;
extern crate rusoto_apigateway;

use rusoto_apigateway::{ApiGateway, ApiGatewayClient, GetRestApisRequest};
use rusoto_core::Region;

#[test]
fn should_get_rest_apis() {
    let client = ApiGatewayClient::simple(Region::UsEast1);
    let request = GetRestApisRequest::default();

    client.get_rest_apis(request).sync().unwrap();

}