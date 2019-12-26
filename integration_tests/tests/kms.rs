#![cfg(feature = "kms")]

extern crate rusoto_core;
extern crate rusoto_kms;

use rusoto_core::Region;
use rusoto_kms::{Kms, KmsClient, ListKeysRequest};

#[tokio::test]
async fn should_list_keys() {
    let client = KmsClient::new(Region::UsEast1);
    let request = ListKeysRequest::default();

    client.list_keys(request).await.unwrap();
}
