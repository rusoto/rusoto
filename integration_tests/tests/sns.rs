#![cfg(feature = "sns")]

extern crate rusoto_core;
extern crate rusoto_sns;

use rusoto_sns::{Sns, SnsClient, ListTopicsInput};
use rusoto_core::Region;

#[test]
fn should_list_topics() {
    let client = SnsClient::simple(Region::UsEast1);
    let request = ListTopicsInput::default();

    let result = client.list_topics(&request).sync().unwrap();
    println!("{:#?}", result);
}
