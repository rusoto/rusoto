#![cfg(feature = "sns")]

extern crate rusoto_core;
extern crate rusoto_sns;

use rusoto_core::Region;
use rusoto_sns::{ListTopicsRequest, Sns, SnsClient};

#[test]
fn should_list_topics() {
    let client = SnsClient::new(Region::UsEast1);
    let request = ListTopicsRequest::default();

    let result = client.list_topics(request).sync().unwrap();
    println!("{:#?}", result);
}
