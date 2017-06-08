#![cfg(feature = "config")]

extern crate rusoto_core;
extern crate rusoto_config;

use rusoto_config::{ConfigService, ConfigServiceClient, DescribeConfigRulesRequest,
                     DescribeDeliveryChannelsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_config_rules() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        ConfigServiceClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let request = DescribeConfigRulesRequest::default();

    match client.describe_config_rules(&request) {
        Ok(response) => {
            println!("{:#?}", response);
            assert!(true)
        }
        Err(err) => panic!("Expected OK response, got {:#?}", err),
    };
}

#[test]
fn should_describe_delivery_channels() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        ConfigServiceClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    let request = DescribeDeliveryChannelsRequest::default();

    match client.describe_delivery_channels(&request) {
        Ok(response) => {
            println!("{:#?}", response);
            assert!(true)
        }
        Err(err) => panic!("Expected OK response, got {:#?}", err),
    };
}
