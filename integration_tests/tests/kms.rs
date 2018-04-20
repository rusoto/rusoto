#![cfg(feature = "kms")]

extern crate rusoto_core;
extern crate rusoto_kms;

use rusoto_kms::{Kms, KmsClient, ListKeysRequest};
use rusoto_core::Region;

#[test]
fn should_list_keys() {
    let client = KmsClient::simple(Region::UsEast1);
    let request = ListKeysRequest::default();

    client.list_keys(request).sync().unwrap();
}
