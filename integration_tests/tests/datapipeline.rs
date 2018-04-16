#![cfg(feature = "datapipeline")]

extern crate rusoto_core;
extern crate rusoto_datapipeline;

use rusoto_datapipeline::{DataPipeline, DataPipelineClient, ListPipelinesInput};
use rusoto_core::Region;

#[test]
fn should_list_pipelines() {
    let client = DataPipelineClient::simple(Region::UsEast1);
    let request = ListPipelinesInput::default();

    client.list_pipelines(request).sync().unwrap();
}
