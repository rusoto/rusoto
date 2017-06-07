#![cfg(feature = "storagegateway")]

extern crate rusoto_core;
extern crate rusoto_storagegateway;

use rusoto_storagegateway::{StorageGateway, StorageGatewayClient, ListGatewaysInput};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_gateways() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        StorageGatewayClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListGatewaysInput::default();

    client.list_gateways(&request).unwrap();
}
