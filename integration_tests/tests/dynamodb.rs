#![cfg(feature = "dynamodb")]

extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::{Region, RusotoError};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

#[tokio::test]
async fn should_parse_error_type() {
    let client = DynamoDbClient::new(Region::UsEast1);

    // limit of -1 should generate a validation error
    let request = ListTablesInput {
        limit: Some(-1),
        ..Default::default()
    };

    let response = client.list_tables(request).await;
    match response {
        Err(RusotoError::Validation(msg)) => {
            // local dynamodb gives a different error, this matches both:
            assert!(msg.contains("greater than or equal to 1"))
        }
        _ => panic!("Should have been a Validation error"),
    };
}

#[tokio::test]
async fn should_list_tables() {
    let client = DynamoDbClient::new(Region::UsEast1);
    let request = ListTablesInput::default();

    client.list_tables(request).await.unwrap();
}
