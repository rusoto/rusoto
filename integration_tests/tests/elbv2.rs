#![cfg(feature = "elbv2")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_elbv2;

use rusoto_elbv2::{Elb, ElbClient, DescribeLoadBalancersInput};
use rusoto_core::Region;

#[test]
fn should_describe_load_balancers() {
    let _ = env_logger::try_init();
    let client = ElbClient::simple(Region::UsEast1);
    let request = DescribeLoadBalancersInput::default();

    let result = client.describe_load_balancers(request).sync().unwrap();
    println!("{:#?}", result);
}
