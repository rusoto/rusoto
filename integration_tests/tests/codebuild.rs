#![cfg(feature = "codebuild")]

extern crate rusoto_codebuild;
extern crate rusoto_core;

use rusoto_codebuild::{CodeBuild, CodeBuildClient, ListProjectsInput};
use rusoto_core::Region;

#[tokio::test]
async fn should_list_projects() {
    let client = CodeBuildClient::new(Region::UsEast1);
    let request = ListProjectsInput::default();

    let result = client.list_projects(request).await.unwrap();
    println!("{:#?}", result);
}
