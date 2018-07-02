#![cfg(feature = "shield")]
extern crate rusoto_core;
extern crate rusoto_shield;

use rusoto_shield::{Shield, ShieldClient, ListAttacksRequest};
use rusoto_core::Region;

#[test]
fn should_list_attacks() {
    let client = ShieldClient::new(Region::UsEast1);
    let request = ListAttacksRequest::default();

    let result = client.list_attacks(request).sync().unwrap();
    println!("{:#?}", result);
}
