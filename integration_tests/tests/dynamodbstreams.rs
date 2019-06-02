#![cfg(feature = "dynamodbstreams")]

extern crate rusoto_core;
extern crate rusoto_dynamodbstreams;

use rusoto_core::Region;
use rusoto_dynamodbstreams::{DynamoDbStreams, DynamoDbStreamsClient, ListStreamsRequest};

#[test]
fn should_list_streams() {
    let client = DynamoDbStreamsClient::new(Region::UsEast1);
    let request = ListStreamsRequest::default();

    client.list_streams(request).sync().unwrap();
}
