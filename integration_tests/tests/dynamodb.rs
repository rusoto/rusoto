#![cfg(feature = "dynamodb")]

extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput, ListTablesError};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_parse_error_type() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let a_region = Region::Custom("localhost:8000".to_owned());
    let client = DynamoDbClient::new(default_tls_client().unwrap(), credentials, a_region);

    // limit of -1 should generate a validation error
    let request = ListTablesInput { limit: Some(-1), ..Default::default() };

    let response = client.list_tables(&request);
    match response {
        Err(ListTablesError::Validation(msg)) => {
            // local dynamodb gives a different error
            println!("msg is {:?}", msg);
            assert!(msg.contains("Member must have value greater than or equal to 1"))
        }
        _ => panic!("Should have been a Validation error"),
    };
}

#[test]
fn should_list_tables() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let a_region = Region::Custom("https://localhost:8000".to_owned());
    let client = DynamoDbClient::new(default_tls_client().unwrap(), credentials, a_region);
    let request = ListTablesInput::default();

    client.list_tables(&request).unwrap();
}
