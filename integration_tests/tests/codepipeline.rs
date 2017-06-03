#![cfg(feature = "codepipeline")]

extern crate rusoto;

use rusoto::codepipeline::{CodePipeline, CodePipelineClient, ListPipelinesInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_pipelines() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        CodePipelineClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListPipelinesInput::default();

    client.list_pipelines(&request).unwrap();
}
