#![cfg(feature = "dynamodbstreams")]

extern crate rusoto_core;
extern crate rusoto_dynamodbstreams;

use rusoto_dynamodbstreams::{DynamoDbStreams, DynamoDbStreamsClient, ListStreamsInput};
use rusoto_core::Region;

#[test]
fn should_list_streams() {
    let client = DynamoDbStreamsClient::simple(Region::UsEast1);
    let request = ListStreamsInput::default();

    client.list_streams(&request).sync().unwrap();
}
