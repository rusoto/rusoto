#![cfg(feature = "datapipeline")]

extern crate rusoto;

use rusoto::datapipeline::{DataPipelineClient, ListPipelinesInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_pipelines() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        DataPipelineClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListPipelinesInput::default();

    client.list_pipelines(&request).unwrap();
}
