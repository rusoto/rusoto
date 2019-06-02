#![cfg(feature = "firehose")]

extern crate rusoto_core;
extern crate rusoto_firehose;

use rusoto_core::Region;
use rusoto_firehose::{KinesisFirehose, KinesisFirehoseClient, ListDeliveryStreamsRequest};

#[test]
fn should_list_delivery_streams() {
    let client = KinesisFirehoseClient::new(Region::UsEast1);
    let request = ListDeliveryStreamsRequest::default();

    client.list_delivery_streams(request).sync().unwrap();
}
