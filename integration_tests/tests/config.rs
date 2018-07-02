#![cfg(feature = "config")]

extern crate rusoto_core;
extern crate rusoto_config;

use rusoto_config::{ConfigService, ConfigServiceClient, DescribeConfigRulesRequest,
                     DescribeDeliveryChannelsRequest};
use rusoto_core::Region;

#[test]
fn should_describe_config_rules() {
    let client = ConfigServiceClient::new(Region::UsEast1);

    let request = DescribeConfigRulesRequest::default();

    match client.describe_config_rules(request).sync() {
        Ok(response) => {
            println!("{:#?}", response);
            assert!(true)
        }
        Err(err) => panic!("Expected OK response, got {:#?}", err),
    };
}

#[test]
fn should_describe_delivery_channels() {
    let client = ConfigServiceClient::new(Region::UsEast1);

    let request = DescribeDeliveryChannelsRequest::default();

    match client.describe_delivery_channels(request).sync() {
        Ok(response) => {
            println!("{:#?}", response);
            assert!(true)
        }
        Err(err) => panic!("Expected OK response, got {:#?}", err),
    };
}
