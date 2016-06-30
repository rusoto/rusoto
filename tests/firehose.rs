#![cfg(feature = "firehose")]

extern crate rusoto;

use rusoto::firehose::{KinesisFirehoseClient, ListDeliveryStreamsInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_delivery_streams() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = KinesisFirehoseClient::new(credentials, Region::UsEast1);
    let request = ListDeliveryStreamsInput::default();

    client.list_delivery_streams(&request).unwrap();
}
