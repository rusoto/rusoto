#![cfg(feature = "route53")]

extern crate rusoto_core;
extern crate rusoto_route53;

use rusoto_core::Region;
use rusoto_route53::{ListHostedZonesRequest, Route53, Route53Client};

#[tokio::test]
async fn should_list_hosted_zones() {
    let client = Route53Client::new(Region::UsEast1);
    let request = ListHostedZonesRequest::default();

    client.list_hosted_zones(request).await.unwrap();
}
