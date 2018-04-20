#![cfg(feature = "workspaces")]

extern crate rusoto_core;
extern crate rusoto_workspaces;

use rusoto_workspaces::{Workspaces, WorkspacesClient, DescribeWorkspacesRequest};
use rusoto_core::Region;

#[test]
fn should_describe_workspaces() {
    let client = WorkspacesClient::simple(Region::UsEast1);
    let request = DescribeWorkspacesRequest::default();

    client.describe_workspaces(request).sync().unwrap();
}
