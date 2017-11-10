#![cfg(feature = "clouddirectory")]

extern crate rusoto_core;
extern crate rusoto_clouddirectory;

use rusoto_clouddirectory::{CloudDirectory, CloudDirectoryClient, ListDirectoriesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_directories() {
    let client = CloudDirectoryClient::new(default_tls_client().unwrap(),
                                           DefaultCredentialsProvider::new().unwrap(),
                                           Region::UsEast1);
    let request = ListDirectoriesRequest::default();

    let result = client.list_directories(&request).sync().unwrap();
    println!("{:#?}", result);
}
