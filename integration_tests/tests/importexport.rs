#![cfg(feature = "importexport")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_importexport;

use rusoto_core::Region;
use rusoto_importexport::{ImportExport, ImportExportClient, ListJobsInput};

#[tokio::test]
#[ignore]
async fn should_list_jobs() {
    let _ = env_logger::try_init();
    let client = ImportExportClient::new(Region::UsEast1);
    let request = ListJobsInput::default();

    let result = client.list_jobs(request).await.unwrap();
    println!("{:#?}", result);
}
