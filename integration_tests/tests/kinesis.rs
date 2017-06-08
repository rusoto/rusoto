#![cfg(feature = "kinesis")]

extern crate rusoto_core;
extern crate rusoto_kinesis;

use rusoto_kinesis::{Kinesis, KinesisClient, ListStreamsInput};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_streams() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = KinesisClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListStreamsInput::default();

    client.list_streams(&request).unwrap();
}
