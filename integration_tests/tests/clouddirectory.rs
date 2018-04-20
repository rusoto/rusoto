#![cfg(feature = "clouddirectory")]

extern crate rusoto_core;
extern crate rusoto_clouddirectory;

use rusoto_clouddirectory::{CloudDirectory, CloudDirectoryClient, ListDirectoriesRequest};
use rusoto_core::Region;

#[test]
fn should_list_directories() {
    let client = CloudDirectoryClient::simple(Region::UsEast1);
    let request = ListDirectoriesRequest::default();

    let result = client.list_directories(request).sync().unwrap();
    println!("{:#?}", result);
}
