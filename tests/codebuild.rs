#![cfg(feature = "codebuild")]

extern crate rusoto;
extern crate env_logger;

use rusoto::codebuild::{CodeBuildClient, ListProjectsInput};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_projects() {
	let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodeBuildClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListProjectsInput::default();

    let result = client.list_projects(&request).unwrap();
	println!("{:#?}", result);
}


