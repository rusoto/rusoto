#![cfg(feature = "workspaces")]

extern crate rusoto_core;
extern crate rusoto_workspaces;

use rusoto_workspaces::{Workspaces, WorkspacesClient, DescribeWorkspacesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_workspaces() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = WorkspacesClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeWorkspacesRequest::default();

    client.describe_workspaces(&request).unwrap();
}
