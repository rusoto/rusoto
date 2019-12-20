#![cfg(feature = "resourcegroupstaggingapi")]

extern crate rusoto_core;
extern crate rusoto_resourcegroupstaggingapi;

use rusoto_core::Region;
use rusoto_resourcegroupstaggingapi::{
    GetResourcesInput, ResourceGroupsTaggingApi, ResourceGroupsTaggingApiClient,
};

#[tokio::test]
async fn should_get_resources() {
    let client = ResourceGroupsTaggingApiClient::new(Region::UsEast1);
    let request = GetResourcesInput::default();

    let result = client.get_resources(request).await.unwrap();
    println!("{:#?}", result);
}
