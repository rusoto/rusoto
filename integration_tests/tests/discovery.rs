#![cfg(feature = "discovery")]

extern crate rusoto_core;
extern crate rusoto_discovery;
extern crate env_logger;
extern crate log;

use rusoto_discovery::{Discovery, DiscoveryClient, DescribeTagsRequest, ListConfigurationsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_tags() {
    let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DiscoveryClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    let request = DescribeTagsRequest::default();

    // Getting an access denied error
    let response = client.describe_tags(&request).unwrap();
    println!("Response: {:?}", response);
}

#[test]
fn should_list_configurations() {
    let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DiscoveryClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    let request = ListConfigurationsRequest{
        configuration_type: "SERVER".to_owned(),
        ..Default::default()
    };

    // Getting an access denied error
    let response = client.list_configurations(&request).unwrap();
    println!("Response: {:?}", response);
}