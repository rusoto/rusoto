#![cfg(feature = "route53domains")]

extern crate rusoto_core;
extern crate rusoto_route53domains;

use rusoto_route53domains::{Route53Domains, Route53DomainsClient, ListOperationsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_operations() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        Route53DomainsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListOperationsRequest::default();

    client.list_operations(&request).sync().unwrap();
}
