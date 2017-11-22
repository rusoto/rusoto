#![cfg(feature = "resourcegroupstaggingapi")]

extern crate rusoto_core;
extern crate rusoto_resourcegroupstaggingapi;

use rusoto_resourcegroupstaggingapi::{ResourceGroupsTaggingApi, ResourceGroupsTaggingApiClient, GetResourcesInput};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_get_resources() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = ResourceGroupsTaggingApiClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = GetResourcesInput::default();

    let result = client.get_resources(&request).unwrap();
	println!("{:#?}", result);
}