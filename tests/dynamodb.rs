#![cfg(feature = "dynamodb")]

extern crate rusoto;

use rusoto::dynamodb::{DynamoDbClient, ListTablesInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_tables() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DynamoDbClient::new(credentials, Region::UsEast1);

    let request = ListTablesInput::default();

    match client.list_tables(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}

