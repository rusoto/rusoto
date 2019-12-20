#![cfg(feature = "pricing")]

extern crate rusoto_core;
extern crate rusoto_pricing;

use rusoto_core::Region;
use rusoto_pricing::{DescribeServicesRequest, Pricing, PricingClient};

#[tokio::test]
async fn should_describe_services() {
    let client = PricingClient::new(Region::UsEast1);
    let request = DescribeServicesRequest::default();

    match client.describe_services(request).await {
        Err(e) => panic!("Couldn't describe services: {}", e),
        Ok(result) => println!("Services: {:?}", result),
    }
}
