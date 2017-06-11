#![cfg(feature = "codebuild")]

extern crate rusoto_core;
extern crate rusoto_codebuild;

use rusoto_codebuild::{CodeBuild, CodeBuildClient, ListProjectsInput};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_projects() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodeBuildClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListProjectsInput::default();

    let result = client.list_projects(&request).unwrap();
	println!("{:#?}", result);
}