#![cfg(feature = "codepipeline")]

extern crate rusoto_core;
extern crate rusoto_codepipeline;

use rusoto_codepipeline::{CodePipeline, CodePipelineClient, ListPipelinesInput};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_pipelines() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        CodePipelineClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListPipelinesInput::default();

    client.list_pipelines(&request).sync().unwrap();
}
