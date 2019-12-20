#![cfg(feature = "elbv2")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_elbv2;

use rusoto_core::Region;
use rusoto_elbv2::{DescribeLoadBalancersInput, Elb, ElbClient};

#[tokio::test]
async fn should_describe_load_balancers() {
    let _ = env_logger::try_init();
    let client = ElbClient::new(Region::UsEast1);
    let request = DescribeLoadBalancersInput::default();

    let result = client.describe_load_balancers(request).await.unwrap();
    println!("{:#?}", result);
}
