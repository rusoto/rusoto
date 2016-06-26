#![cfg(feature = "dynamodbstreams")]

extern crate rusoto;

use rusoto::dynamodbstreams::{DynamoDbStreamsClient, ListStreamsInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_streams() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DynamoDbStreamsClient::new(credentials, Region::UsEast1);
    let request = ListStreamsInput::default();

    client.list_streams(&request).unwrap();
}

