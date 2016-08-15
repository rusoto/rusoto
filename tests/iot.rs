#![cfg(feature = "iot")]

extern crate rusoto;
extern crate env_logger;
#[macro_use] extern crate log;
use rusoto::iot::{IotClient, ListThingsRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_things() {
	let _ = env_logger::init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = IotClient::new(credentials, Region::UsEast1);
    let request = ListThingsRequest::default();

    client.list_things(&request).unwrap();
}
