#![cfg(feature = "codepipeline")]

extern crate rusoto;

use rusoto::codepipeline::{CodePipelineClient, ListPipelinesInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_pipelines() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodePipelineClient::new(credentials, Region::UsEast1);
    let request = ListPipelinesInput::default();

    client.list_pipelines(&request).unwrap();
}
