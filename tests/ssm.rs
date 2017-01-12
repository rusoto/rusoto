#![cfg(feature = "ssm")]

extern crate rusoto;

use rusoto::ssm::{SsmClient, ListDocumentsRequest, ListCommandsRequest, ListCommandInvocationsRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_documents() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SsmClient::new(credentials, Region::UsEast1).unwrap();
    let request = ListDocumentsRequest::default();

    client.list_documents(&request).unwrap();
}

#[test]
fn should_list_commands() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SsmClient::new(credentials, Region::UsEast1).unwrap();
    let request = ListCommandsRequest::default();

    client.list_commands(&request).unwrap();
}

#[test]
fn should_list_command_invocations() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SsmClient::new(credentials, Region::UsEast1).unwrap();
    let request = ListCommandInvocationsRequest::default();

    client.list_command_invocations(&request).unwrap();
}