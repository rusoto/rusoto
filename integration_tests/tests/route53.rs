#![cfg(feature = "route53")]

extern crate rusoto_core;
extern crate rusoto_route53;

use rusoto_route53::{Route53, Route53Client, ListHostedZonesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_hosted_zones() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        Route53Client::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListHostedZonesRequest::default();

    client.list_hosted_zones(&request).sync().unwrap();
}
