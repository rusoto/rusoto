#![cfg(feature = "ssm")]

extern crate rusoto_core;
extern crate rusoto_ssm;

use rusoto_ssm::{Ssm, SsmClient, ListDocumentsRequest, ListCommandsRequest,
                  ListCommandInvocationsRequest};
use rusoto_core::Region;

#[test]
fn should_list_documents() {
    let client = SsmClient::simple(Region::UsEast1);
    let request = ListDocumentsRequest::default();

    client.list_documents(request).sync().unwrap();
}

#[test]
fn should_list_commands() {
    let client = SsmClient::simple(Region::UsEast1);
    let request = ListCommandsRequest::default();

    client.list_commands(request).sync().unwrap();
}

#[test]
fn should_list_command_invocations() {
    let client = SsmClient::simple(Region::UsEast1);
    let request = ListCommandInvocationsRequest::default();

    client.list_command_invocations(request).sync().unwrap();
}
