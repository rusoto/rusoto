#![cfg(feature = "apigateway")]

extern crate rusoto_apigateway;
extern crate rusoto_core;

use rusoto_apigateway::{ApiGateway, ApiGatewayClient, GetRestApisRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_get_rest_apis() {
    let client = ApiGatewayClient::new(Region::UsEast1);
    let request = GetRestApisRequest::default();

    client.get_rest_apis(request).await.unwrap();
}
