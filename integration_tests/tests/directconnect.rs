#![cfg(feature = "directconnect")]

extern crate rusoto_core;
extern crate rusoto_directconnect;

use rusoto_core::{Region, RusotoError};
use rusoto_directconnect::{
    DescribeConnectionsError, DescribeConnectionsRequest, DirectConnect, DirectConnectClient,
};

#[tokio::test]
async fn should_describe_connections() {
    let client = DirectConnectClient::new(Region::UsEast1);
    let request = DescribeConnectionsRequest::default();

    client.describe_connections(request).await.unwrap();
}

#[tokio::test]
async fn should_fail_gracefully() {
    let client = DirectConnectClient::new(Region::UsEast1);

    let request = DescribeConnectionsRequest {
        connection_id: Some("invalid".to_string()),
    };

    match client.describe_connections(request).await {
        Err(RusotoError::Service(DescribeConnectionsError::DirectConnectClient(msg))) => {
            assert!(msg.contains("Connection ID"))
        }
        err @ _ => panic!("Expected DirectConnectClient error, got {:#?}", err),
    };
}

#[tokio::test]
async fn should_describe_locations() {
    let client = DirectConnectClient::new(Region::UsEast1);

    client.describe_locations().await.unwrap();
}

#[tokio::test]
async fn should_describe_virtual_gateways() {
    let client = DirectConnectClient::new(Region::UsEast1);

    client.describe_virtual_gateways().await.unwrap();
}
