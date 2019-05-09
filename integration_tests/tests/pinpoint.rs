#![cfg(feature = "pinpoint")]
extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_pinpoint;

use rusoto_core::Region;
use rusoto_pinpoint::{GetAppsRequest, Pinpoint, PinpointClient};

#[test]
fn should_get_apps() {
    env_logger::init();
    let client = PinpointClient::new(Region::UsEast1);
    let request = GetAppsRequest::default();

    println!("{:?}", client.get_apps(request).sync().unwrap());
}
