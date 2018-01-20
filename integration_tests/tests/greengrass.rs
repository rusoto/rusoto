#![cfg(feature = "greengrass")]

extern crate rusoto_core;
extern crate rusoto_greengrass;
extern crate env_logger;

use rusoto_greengrass::{GreenGrass, GreenGrassClient, ListGroupsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_groups() {
    env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = GreenGrassClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    
    let request = ListGroupsRequest::default();

    let result = client.list_groups(&request).unwrap();
    println!("{:#?}", result);
}