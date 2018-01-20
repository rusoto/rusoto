#![cfg(feature = "iot")]

extern crate rusoto_core;
extern crate rusoto_iot;
extern crate env_logger;

use rusoto_iot::{Iot, IotClient, ListThingsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_things() {
    let _ = env_logger::try_init();
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = IotClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListThingsRequest::default();

    client.list_things(&request).unwrap();
}
