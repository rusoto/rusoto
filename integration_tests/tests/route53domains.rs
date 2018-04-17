#![cfg(feature = "route53domains")]

extern crate rusoto_core;
extern crate rusoto_route53domains;

use rusoto_route53domains::{Route53Domains, Route53DomainsClient, ListOperationsRequest};
use rusoto_core::Region;

#[test]
fn should_list_operations() {
    let client = Route53DomainsClient::simple(Region::UsEast1);
    let request = ListOperationsRequest::default();

    client.list_operations(request).sync().unwrap();
}
