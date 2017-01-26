#![cfg(feature = "sdb")]
extern crate rusoto;

use rusoto::sdb::{SimpleDbClient, ListDomainsRequest};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_domains() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SimpleDbClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListDomainsRequest::default();

    let result = client.list_domains(&request);
    println!("{:#?}", result);
}


