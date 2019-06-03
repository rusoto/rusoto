#![cfg(feature = "storagegateway")]

extern crate rusoto_core;
extern crate rusoto_storagegateway;

use rusoto_core::Region;
use rusoto_storagegateway::{ListGatewaysRequest, StorageGateway, StorageGatewayClient};

#[test]
fn should_list_gateways() {
    let client = StorageGatewayClient::new(Region::UsEast1);
    let request = ListGatewaysRequest::default();

    client.list_gateways(request).sync().unwrap();
}
