#![cfg(feature = "dynamodbstreams")]

extern crate rusoto;

use rusoto::dynamodbstreams::{DynamoDbStreamsClient, ListStreamsInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_streams() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        DynamoDbStreamsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListStreamsInput::default();

    client.list_streams(&request).unwrap();
}
