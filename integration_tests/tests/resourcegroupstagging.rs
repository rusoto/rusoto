#![cfg(feature = "resourcegroupstaggingapi")]

extern crate rusoto_core;
extern crate rusoto_resourcegroupstaggingapi;

use rusoto_core::Region;
use rusoto_resourcegroupstaggingapi::{
    GetResourcesRequest, ResourceGroupsTaggingApi, ResourceGroupsTaggingApiClient,
};

#[test]
fn should_get_resources() {
    let client = ResourceGroupsTaggingApiClient::new(Region::UsEast1);
    let request = GetResourcesRequest::default();

    let result = client.get_resources(request).sync().unwrap();
    println!("{:#?}", result);
}
