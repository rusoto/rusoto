#![cfg(feature = "route53")]

extern crate rusoto_core;
extern crate rusoto_route53;

use rusoto_route53::{Route53, Route53Client, ListHostedZonesRequest};
use rusoto_core::Region;

#[test]
fn should_list_hosted_zones() {
    let client = Route53Client::simple(Region::UsEast1);
    let request = ListHostedZonesRequest::default();

    client.list_hosted_zones(request).sync().unwrap();
}
