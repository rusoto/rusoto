#![cfg(feature = "config")]

extern crate rusoto_config;
extern crate rusoto_core;

use rusoto_config::{
    ConfigService, ConfigServiceClient, DescribeConfigRulesRequest, DescribeDeliveryChannelsRequest,
};
use rusoto_core::Region;

#[tokio::test]
async fn should_describe_config_rules() {
    let client = ConfigServiceClient::new(Region::UsEast1);

    let request = DescribeConfigRulesRequest::default();

    match client.describe_config_rules(request).await {
        Ok(response) => {
            println!("{:#?}", response);
            assert!(true)
        }
        Err(err) => panic!("Expected OK response, got {:#?}", err),
    };
}

#[tokio::test]
async fn should_describe_delivery_channels() {
    let client = ConfigServiceClient::new(Region::UsEast1);

    let request = DescribeDeliveryChannelsRequest::default();

    match client.describe_delivery_channels(request).await {
        Ok(response) => {
            println!("{:#?}", response);
            assert!(true)
        }
        Err(err) => panic!("Expected OK response, got {:#?}", err),
    };
}
