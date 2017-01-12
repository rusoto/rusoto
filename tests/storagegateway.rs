#![cfg(feature = "storagegateway")]

extern crate rusoto;

use rusoto::storagegateway::{StorageGatewayClient, ListGatewaysInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_gateways() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = StorageGatewayClient::new(credentials, Region::UsEast1).unwrap();
    let request = ListGatewaysInput::default();

    client.list_gateways(&request).unwrap();
}
