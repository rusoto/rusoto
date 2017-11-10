#![cfg(feature = "sdb")]
extern crate rusoto_core;
extern crate rusoto_sdb;

use rusoto_sdb::{SimpleDb, SimpleDbClient, ListDomainsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_domains() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SimpleDbClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListDomainsRequest::default();

    let result = client.list_domains(&request).sync();
    println!("{:#?}", result);
}
