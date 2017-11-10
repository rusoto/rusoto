#![cfg(feature = "shield")]
extern crate rusoto_core;
extern crate rusoto_shield;

use rusoto_shield::{Shield, ShieldClient, ListAttacksRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_attacks() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = ShieldClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListAttacksRequest::default();

    let result = client.list_attacks(&request).sync().unwrap();
    println!("{:#?}", result);
}
