#![cfg(feature = "cloudformation")]

extern crate rusoto_cloudformation;
extern crate rusoto_core;

use rusoto_cloudformation::{CloudFormation, CloudFormationClient, ListStacksRequest};
use rusoto_core::Region;

#[test]
fn should_list_stacks() {
    let client = CloudFormationClient::new(Region::UsEast1);
    let request = ListStacksRequest::default();

    let result = client.list_stacks(request).sync().unwrap();
    println!("{:#?}", result);
}

#[test]
fn should_list_stacks_with_status_filter() {
    let client = CloudFormationClient::new(Region::UsEast1);

    let filters = vec!["CREATE_COMPLETE".to_owned()];
    let request = ListStacksRequest {
        stack_status_filter: Some(filters),
        ..Default::default()
    };

    let result = client.list_stacks(request).sync().unwrap();
    println!("{:#?}", result);
}
