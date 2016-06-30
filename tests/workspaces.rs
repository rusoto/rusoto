#![cfg(feature = "workspaces")]

extern crate rusoto;

use rusoto::workspaces::{WorkspacesClient, DescribeWorkspacesRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_workspaces() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = WorkspacesClient::new(credentials, Region::UsEast1);
    let request = DescribeWorkspacesRequest::default();

    client.describe_workspaces(&request).unwrap();
}
