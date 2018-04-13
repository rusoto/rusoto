#![cfg(feature = "iot")]

extern crate rusoto_core;
extern crate rusoto_iot;
extern crate env_logger;

use rusoto_iot::{Iot, IotClient, ListThingsRequest};
use rusoto_core::Region;

#[test]
fn should_list_things() {
    let _ = env_logger::try_init();
    let client = IotClient::simple(Region::UsEast1);
    let request = ListThingsRequest::default();

    client.list_things(request).sync().unwrap();
}
