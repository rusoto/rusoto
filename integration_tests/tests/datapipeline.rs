#![cfg(feature = "datapipeline")]

extern crate rusoto_core;
extern crate rusoto_datapipeline;

use rusoto_core::Region;
use rusoto_datapipeline::{DataPipeline, DataPipelineClient, ListPipelinesInput};

#[tokio::test]
async fn should_list_pipelines() {
    let client = DataPipelineClient::new(Region::UsEast1);
    let request = ListPipelinesInput::default();

    client.list_pipelines(request).await.unwrap();
}
