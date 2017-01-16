#![cfg(feature = "storagegateway")]

extern crate rusoto;

use rusoto::storagegateway::{StorageGatewayClient, ListGatewaysInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_gateways() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = StorageGatewayClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListGatewaysInput::default();

    client.list_gateways(&request).unwrap();
}
