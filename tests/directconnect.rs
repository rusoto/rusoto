#![cfg(feature = "directconnect")]

extern crate rusoto;

use rusoto::directconnect::{DirectConnectClient, DescribeConnectionsRequest, DescribeConnectionsError};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_connections() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DirectConnectClient::new(credentials, Region::UsEast1).unwrap();
    let request = DescribeConnectionsRequest::default();

    client.describe_connections(&request).unwrap();
}

#[test]
fn should_fail_gracefully() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DirectConnectClient::new(credentials, Region::UsEast1).unwrap();

    let request = DescribeConnectionsRequest {
        connection_id: Some("invalid".to_string())
    };

    match client.describe_connections(&request) {
        Err(DescribeConnectionsError::DirectConnectClient(msg)) => assert!(msg.contains("Connection ID")),
        err @ _ => panic!("Expected DirectConnectClient error, got {:#?}", err)
    };
}

#[test]
fn should_describe_locations() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DirectConnectClient::new(credentials, Region::UsEast1).unwrap();

    client.describe_locations().unwrap();
}

#[test]
fn should_describe_virtual_gateways() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DirectConnectClient::new(credentials, Region::UsEast1).unwrap();

    client.describe_virtual_gateways().unwrap();
}
