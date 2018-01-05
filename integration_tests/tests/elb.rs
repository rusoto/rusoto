#![cfg(feature = "elb")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_elb;

use rusoto_elb::{Elb, ElbClient, DescribeAccessPointsInput};
use rusoto_core::Region;

#[test]
fn should_describe_load_balancers() {
    let _ = env_logger::try_init();
    let client = ElbClient::simple(Region::UsEast1);
    let request = DescribeAccessPointsInput::default();

    let result = client.describe_load_balancers(&request).sync();
    println!("{:#?}", result);
}
