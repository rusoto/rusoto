#![cfg(feature = "config")]

extern crate rusoto;

use rusoto::config::{ConfigServiceClient, DescribeConfigRulesRequest,
                     DescribeDeliveryChannelsRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

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
