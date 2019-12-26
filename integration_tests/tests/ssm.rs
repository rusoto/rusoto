#![cfg(feature = "ssm")]

extern crate rusoto_core;
extern crate rusoto_ssm;

use rusoto_core::Region;
use rusoto_ssm::{
    ListCommandInvocationsRequest, ListCommandsRequest, ListDocumentsRequest, Ssm, SsmClient,
};

#[tokio::test]
async fn should_list_documents() {
    let client = SsmClient::new(Region::UsEast1);
    let request = ListDocumentsRequest::default();

    client.list_documents(request).await.unwrap();
}

#[tokio::test]
async fn should_list_commands() {
    let client = SsmClient::new(Region::UsEast1);
    let request = ListCommandsRequest::default();

    client.list_commands(request).await.unwrap();
}

#[tokio::test]
async fn should_list_command_invocations() {
    let client = SsmClient::new(Region::UsEast1);
    let request = ListCommandInvocationsRequest::default();

    client.list_command_invocations(request).await.unwrap();
}
