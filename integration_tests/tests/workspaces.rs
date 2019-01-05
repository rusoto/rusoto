#![cfg(feature = "workspaces")]

extern crate rusoto_core;
extern crate rusoto_workspaces;

use rusoto_core::Region;
use rusoto_workspaces::{DescribeWorkspacesRequest, Workspaces, WorkspacesClient};

#[test]
fn should_describe_workspaces() {
    let client = WorkspacesClient::new(Region::UsEast1);
    let request = DescribeWorkspacesRequest::default();

    client.describe_workspaces(request).sync().unwrap();
}
