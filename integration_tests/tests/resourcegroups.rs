#![cfg(feature = "resource-groups")]

extern crate rusoto_core;
extern crate rusoto_resource_groups;

use rusoto_core::Region;
use rusoto_resource_groups::{ListGroupsInput, ResourceGroups, ResourceGroupsClient};

#[tokio::test]
async fn should_list_groups() {
    let client = ResourceGroupsClient::new(Region::UsEast1);
    let request = ListGroupsInput::default();

    let result = client.list_groups(request).await.unwrap();
    println!("{:#?}", result);
    assert!(result.group_identifiers.is_some());
}
