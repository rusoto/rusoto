#![cfg(feature = "codepipeline")]

extern crate rusoto_codepipeline;
extern crate rusoto_core;

use rusoto_codepipeline::{CodePipeline, CodePipelineClient, ListPipelinesInput};
use rusoto_core::Region;

#[tokio::test]
async fn should_list_pipelines() {
    let client = CodePipelineClient::new(Region::UsEast1);
    let request = ListPipelinesInput::default();

    client.list_pipelines(request).await.unwrap();
}
