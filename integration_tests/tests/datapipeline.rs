#![cfg(feature = "datapipeline")]

extern crate rusoto_core;
extern crate rusoto_datapipeline;

use rusoto_datapipeline::{DataPipeline, DataPipelineClient, ListPipelinesInput};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_pipelines() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        DataPipelineClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListPipelinesInput::default();

    client.list_pipelines(&request).unwrap();
}
