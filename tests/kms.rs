#![cfg(feature = "kms")]

extern crate rusoto;

use rusoto::kms::{KmsClient, ListKeysRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_keys() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = KmsClient::new(credentials, Region::UsEast1);
    let request = ListKeysRequest::default();

    client.list_keys(&request).unwrap();
}
