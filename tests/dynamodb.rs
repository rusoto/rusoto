#![cfg(feature = "dynamodb")]

extern crate rusoto;

use rusoto::dynamodb::{DynamoDbClient, ListTablesInput, ListTablesError};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_parse_error_type() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DynamoDbClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);

    // limit of -1 should generate a validation error
    let request = ListTablesInput { limit: Some(-1), ..Default::default() };

    let response = client.list_tables(&request);
    match response {
        Err(ListTablesError::Validation(msg)) => {
            assert!(msg.contains("Member must have value greater than or equal to 1"))
        }
        _ => panic!("Should have been a Validation error"),
    };
}

#[test]
fn should_list_tables() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DynamoDbClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListTablesInput::default();

    client.list_tables(&request).unwrap();
}
