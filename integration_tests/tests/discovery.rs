#![cfg(feature = "discovery")]

extern crate env_logger;
extern crate log;
extern crate rusoto_core;
extern crate rusoto_discovery;

use rusoto_core::{Region, RusotoError};
use rusoto_discovery::{
    DescribeTagsRequest, Discovery, DiscoveryClient, ListConfigurationsRequest,
};

use std::str;

// These tests require the calling AWS account to be whitelisted.
// See http://docs.aws.amazon.com/application-discovery/latest/userguide/console_walkthrough.html
// For now we'll accept the error message returned if the account is not whitelisted.
#[tokio::test]
async fn should_describe_tags() {
    let _ = env_logger::try_init();

    let client = DiscoveryClient::new(Region::UsWest2);
    let request = DescribeTagsRequest::default();

    match client.describe_tags(request).await {
        Ok(response) => println!("Response: {:?}", response),
        Err(e) => {
            println!("Got expected error of {}", e);
            match e {
                RusotoError::Unknown(ref e) => {
                    assert!(str::from_utf8(&e.body)
                        .unwrap()
                        .contains("is not whitelisted to access"));
                }
                _ => panic!("Error from Discovery service should be typed."),
            }
        }
    }
}

#[tokio::test]
async fn should_list_configurations() {
    let _ = env_logger::try_init();

    let client = DiscoveryClient::new(Region::UsWest2);
    let request = ListConfigurationsRequest {
        configuration_type: "SERVER".to_owned(),
        ..Default::default()
    };

    match client.list_configurations(request).await {
        Ok(response) => println!("Response: {:?}", response),
        Err(e) => match e {
            RusotoError::Unknown(ref e) => {
                assert!(str::from_utf8(&e.body)
                    .unwrap()
                    .contains("is not whitelisted to access"));
            }
            _ => panic!("Error from Discovery service should be typed."),
        },
    }
}
