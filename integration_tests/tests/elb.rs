#![cfg(feature = "elb")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_elb;

use rusoto_elb::{Elb, ElbClient, DescribeAccessPointsInput};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_describe_load_balancers() {
    let _ = env_logger::try_init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = ElbClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = DescribeAccessPointsInput::default();

    let result = client.describe_load_balancers(&request);
    println!("{:#?}", result);
}
