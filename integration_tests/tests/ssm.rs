#![cfg(feature = "ssm")]

extern crate rusoto_core;
extern crate rusoto_ssm;

use rusoto_ssm::{Ssm, SsmClient, ListDocumentsRequest, ListCommandsRequest,
                  ListCommandInvocationsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_documents() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SsmClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListDocumentsRequest::default();

    client.list_documents(&request).sync().unwrap();
}

#[test]
fn should_list_commands() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SsmClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListCommandsRequest::default();

    client.list_commands(&request).sync().unwrap();
}

#[test]
fn should_list_command_invocations() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SsmClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListCommandInvocationsRequest::default();

    client.list_command_invocations(&request).sync().unwrap();
}
