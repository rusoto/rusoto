#![cfg(feature = "resource-groups")]

extern crate rusoto_core;
extern crate rusoto_resource_groups;

use rusoto_core::Region;
use rusoto_resource_groups::{ListGroupsRequest, ResourceGroups, ResourceGroupsClient};

#[test]
fn should_list_groups() {
    let client = ResourceGroupsClient::new(Region::UsEast1);
    let request = ListGroupsRequest::default();

    let result = client.list_groups(request).sync().unwrap();
    println!("{:#?}", result);
    assert!(result.group_identifiers.is_some());
}
