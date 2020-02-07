#![cfg(feature = "route53domains")]

extern crate rusoto_core;
extern crate rusoto_route53domains;

use rusoto_core::Region;
use rusoto_route53domains::{ListOperationsRequest, Route53Domains, Route53DomainsClient};

#[tokio::test]
async fn should_list_operations() {
    let client = Route53DomainsClient::new(Region::UsEast1);
    let request = ListOperationsRequest::default();

    client.list_operations(request).await.unwrap();
}
