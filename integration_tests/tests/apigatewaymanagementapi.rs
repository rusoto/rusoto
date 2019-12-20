#![cfg(feature = "apigatewaymanagementapi")]

extern crate rusoto_apigatewaymanagementapi;
extern crate rusoto_core;

use rusoto_apigatewaymanagementapi::{
    ApiGatewayManagementApi, ApiGatewayManagementApiClient, PostToConnectionRequest,
};
use rusoto_core::Region;

#[tokio::test]
async fn should_work() {
    // post_to_connection is a bit like invoke using the following method
    // https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-call-api.html
    // except this is intended for websocket connection postings over rest
    //
    // you will need to set the endpoint for this client
    // typically in the form `https://{api-id}.execute-api.{region}.amazonaws.com/{stage}` as
    // documented here
    // https://docs.aws.amazon.com/cli/latest/reference/apigatewaymanagementapi/index.html#cli-aws-apigatewaymanagementapi
    let client = ApiGatewayManagementApiClient::new(Region::Custom {
        name: "us-east1".to_owned(),
        endpoint: "https://123.execute-api.us-east1.amazonaws.com/dev/".to_owned(),
    });
    let response = client
        .post_to_connection(PostToConnectionRequest {
            connection_id: "bogus".into(),
            ..PostToConnectionRequest::default()
        })
        .await;
    // we expect an error because we're posting to an apigw that does not exist
    assert!(response.is_err());
    println!("response is {:#?}", response);
}
