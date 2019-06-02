#![cfg(feature = "codebuild")]

extern crate rusoto_codebuild;
extern crate rusoto_core;

use rusoto_codebuild::{CodeBuild, CodeBuildClient, ListProjectsRequest};
use rusoto_core::Region;

#[test]
fn should_list_projects() {
    let client = CodeBuildClient::new(Region::UsEast1);
    let request = ListProjectsRequest::default();

    let result = client.list_projects(request).sync().unwrap();
    println!("{:#?}", result);
}
