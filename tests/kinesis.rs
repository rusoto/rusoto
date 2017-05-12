#![cfg(feature = "kinesis")]

extern crate rusoto;

use rusoto::kinesis::{Kinesis, KinesisClient, ListStreamsInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_streams() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = KinesisClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListStreamsInput::default();

    client.list_streams(&request).unwrap();
}
