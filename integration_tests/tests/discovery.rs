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

    // The AWS CLI also returns this error:
    match client.describe_tags(&request) {
        Ok(response) => println!("Response: {:?}", response),
        Err(e) => assert!(format!("{}", e).contains("is not whitelisted to access")),
    }
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

    // The AWS CLI also returns this error:
    match client.list_configurations(&request) {
        Ok(response) => println!("Response: {:?}", response),
        Err(e) => assert!(format!("{}", e).contains("is not whitelisted to access")),
    }
}