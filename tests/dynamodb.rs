#![cfg(feature = "dynamodb")]

extern crate rusoto;
extern crate tokio_core;

use tokio_core::reactor::Core;
use rusoto::dynamodb::{DynamoDbClient, ListTablesInput, ListTablesError};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_parse_error_type() {
    let mut core = Core::new().unwrap();

    let credentials = DefaultCredentialsProvider::new().unwrap();
    let tls_client = default_tls_client(1, &core.handle()).unwrap();
    let client = DynamoDbClient::new(tls_client, credentials, Region::UsEast1);

    // limit of -1 should generate a validation error
    let request = ListTablesInput { limit: Some(-1), ..Default::default() };

    let test = client.list_tables(&request);

    match core.run(test) {
        Err(ListTablesError::Validation(msg)) => {
            assert!(msg.contains("Member must have value greater than or equal to 1"))
        }
        _ => panic!("Should have been a Validation error"),
    };
}

#[test]
fn should_list_tables() {
    let mut core = Core::new().unwrap();

    let credentials = DefaultCredentialsProvider::new().unwrap();
    let tls_client = default_tls_client(1, &core.handle()).unwrap();
    let client = DynamoDbClient::new(tls_client, credentials, Region::UsEast1);
    let request = ListTablesInput::default();

    let response = client.list_tables(&request);

    core.run(response).unwrap();
}
