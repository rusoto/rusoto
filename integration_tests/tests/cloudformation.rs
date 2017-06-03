#![cfg(feature = "cloudformation")]

extern crate rusoto;

use rusoto::cloudformation::{CloudFormation, CloudFormationClient, ListStacksInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_stacks() {
    let client = CloudFormationClient::new(default_tls_client().unwrap(),
                                           DefaultCredentialsProvider::new().unwrap(),
                                           Region::UsEast1);
    let request = ListStacksInput::default();

    let result = client.list_stacks(&request).unwrap();
    println!("{:#?}", result);
}

#[test]
fn should_list_stacks_with_status_filter() {
    let client = CloudFormationClient::new(default_tls_client().unwrap(),
                                           DefaultCredentialsProvider::new().unwrap(),
                                           Region::UsEast1);

    let filters = vec!["CREATE_COMPLETE".to_owned()];
    let request = ListStacksInput { stack_status_filter: Some(filters), ..Default::default() };

    let result = client.list_stacks(&request).unwrap();
    println!("{:#?}", result);
}
