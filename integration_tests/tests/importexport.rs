#![cfg(feature = "importexport")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_importexport;

use rusoto_importexport::{ImportExport, ImportExportClient, ListJobsInput};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_jobs() {
    let _ = env_logger::try_init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        ImportExportClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListJobsInput::default();

    let result = client.list_jobs(&request);
    println!("{:#?}", result);
}
