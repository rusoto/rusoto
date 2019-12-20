#![cfg(feature = "appsync")]

extern crate env_logger;
extern crate rusoto_appsync;
extern crate rusoto_core;

use rusoto_appsync::{AppSync, AppSyncClient, ListGraphqlApisRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_list_graphql_apis() {
    let _ = env_logger::try_init();
    let client = AppSyncClient::new(Region::UsEast1);
    let request = ListGraphqlApisRequest::default();

    let result = client.list_graphql_apis(request).await.unwrap();
    println!("{:#?}", result);
}
