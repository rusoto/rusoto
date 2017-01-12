#![cfg(feature = "datapipeline")]

extern crate rusoto;

use rusoto::datapipeline::{DataPipelineClient, ListPipelinesInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_pipelines() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DataPipelineClient::new(credentials, Region::UsEast1).unwrap();
    let request = ListPipelinesInput::default();

    client.list_pipelines(&request).unwrap();
}
