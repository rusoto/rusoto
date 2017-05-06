#![cfg(feature = "elbv2")]
extern crate env_logger;
extern crate rusoto;

use rusoto::elbv2::{Elb, ElbClient, DescribeLoadBalancersInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_describe_load_balancers() {
    let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = ElbClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeLoadBalancersInput::default();

    let result = client.describe_load_balancers(&request);
    println!("{:#?}", result);
}
