#![cfg(feature = "resource-groups")]

extern crate rusoto_core;
extern crate rusoto_resource_groups;

use rusoto_resource_groups::{ResourceGroups, ResourceGroupsClient, ListGroupsInput};
use rusoto_core::Region;

#[test]
fn should_list_groups() {
    let client = ResourceGroupsClient::new(Region::UsEast1);
    let request = ListGroupsInput::default();

    let result = client.list_groups(request).sync().unwrap();
	println!("{:#?}", result);
    assert!(result.groups.is_some());
}