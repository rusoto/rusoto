#![cfg(feature = "firehose")]

extern crate rusoto_core;
extern crate rusoto_firehose;

use rusoto_firehose::{KinesisFirehose, KinesisFirehoseClient, ListDeliveryStreamsInput};
use rusoto_core::Region;

#[test]
fn should_list_delivery_streams() {
    let client = KinesisFirehoseClient::simple(Region::UsEast1);
    let request = ListDeliveryStreamsInput::default();

    client.list_delivery_streams(&request).sync().unwrap();
}
