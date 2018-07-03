#![cfg(feature = "codebuild")]

extern crate rusoto_core;
extern crate rusoto_codebuild;

use rusoto_codebuild::{CodeBuild, CodeBuildClient, ListProjectsInput};
use rusoto_core::Region;

#[test]
fn should_list_projects() {
    let client = CodeBuildClient::new(Region::UsEast1);
    let request = ListProjectsInput::default();

    let result = client.list_projects(request).sync().unwrap();
	println!("{:#?}", result);
}