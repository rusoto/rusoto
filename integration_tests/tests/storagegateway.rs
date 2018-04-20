#![cfg(feature = "storagegateway")]

extern crate rusoto_core;
extern crate rusoto_storagegateway;

use rusoto_storagegateway::{StorageGateway, StorageGatewayClient, ListGatewaysInput};
use rusoto_core::Region;

#[test]
fn should_list_gateways() {
    let client = StorageGatewayClient::simple(Region::UsEast1);
    let request = ListGatewaysInput::default();

    client.list_gateways(request).sync().unwrap();
}
