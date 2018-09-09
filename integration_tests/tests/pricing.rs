#![cfg(feature = "pricing")]

extern crate rusoto_core;
extern crate rusoto_pricing;

use rusoto_core::Region;
use rusoto_pricing::{PricingClient, Pricing, DescribeServicesRequest};

#[test]
fn should_describe_services() {
    let client = PricingClient::new(Region::UsEast1);
    let request = DescribeServicesRequest::default();

    match client.describe_services(request).sync() {
        Err(e) => panic!("Couldn't describe services: {}", e),
        Ok(result) => println!("Services: {:?}", result),
    }
}