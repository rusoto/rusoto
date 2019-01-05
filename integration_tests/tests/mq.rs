#![cfg(feature = "mq")]

extern crate rusoto_core;
extern crate rusoto_mq;

use rusoto_core::Region;
use rusoto_mq::{ListBrokersRequest, MQClient, MQ};

#[test]
fn should_list_brokers() {
    let client = MQClient::new(Region::UsEast1);
    let request = ListBrokersRequest::default();

    let result = client.list_brokers(request).sync().unwrap();
    println!("Results: {:?}", result);
}
