#![cfg(feature = "route53domains")]

extern crate rusoto;

use rusoto::route53domains::{Route53Domains, Route53DomainsClient, ListOperationsRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_operations() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        Route53DomainsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListOperationsRequest::default();

    client.list_operations(&request).unwrap();
}
