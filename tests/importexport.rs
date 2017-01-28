#![cfg(feature = "importexport")]
extern crate env_logger;
extern crate rusoto;

use rusoto::importexport::{ImportExportClient, ListJobsInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_jobs() {
    let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = ImportExportClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListJobsInput::default();

    let result = client.list_jobs(&request);
    println!("{:#?}", result);
}


