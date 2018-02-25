#![cfg(feature = "sdb")]
extern crate rusoto_core;
extern crate rusoto_sdb;

use rusoto_sdb::{SimpleDb, SimpleDbClient, ListDomainsRequest};
use rusoto_core::Region;

#[test]
fn should_list_domains() {
    let client = SimpleDbClient::simple(Region::UsEast1);
    let request = ListDomainsRequest::default();

    let result = client.list_domains(&request).sync().unwrap();
    println!("{:#?}", result);
}
