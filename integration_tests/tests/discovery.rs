#![cfg(feature = "discovery")]

extern crate rusoto_core;
extern crate rusoto_discovery;
extern crate env_logger;
extern crate log;

use rusoto_discovery::{Discovery, DiscoveryClient, DescribeTagsRequest, ListConfigurationsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;


// These tests require the calling AWS account to be whitelisted.
// See http://docs.aws.amazon.com/application-discovery/latest/userguide/console_walkthrough.html
// For now we'll accept the error message returned if the account is not whitelisted.
#[test]
fn should_describe_tags() {
    let _ = env_logger::try_init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DiscoveryClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    let request = DescribeTagsRequest::default();

    match client.describe_tags(&request) {
        Ok(response) => println!("Response: {:?}", response),
        Err(e) => assert!(format!("{}", e).contains("is not whitelisted to access")),
    }
}

#[test]
fn should_list_configurations() {
    let _ = env_logger::try_init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DiscoveryClient::new(default_tls_client().unwrap(), credentials, Region::UsWest2);
    let request = ListConfigurationsRequest{
        configuration_type: "SERVER".to_owned(),
        ..Default::default()
    };

    match client.list_configurations(&request) {
        Ok(response) => println!("Response: {:?}", response),
        Err(e) => assert!(format!("{}", e).contains("is not whitelisted to access")),
    }
}