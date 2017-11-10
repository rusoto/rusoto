#![cfg(feature = "sns")]

extern crate rusoto_core;
extern crate rusoto_sns;

use rusoto_sns::{Sns, SnsClient, ListTopicsInput};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_topics() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SnsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListTopicsInput::default();

    let result = client.list_topics(&request).sync().unwrap();
    println!("{:#?}", result);
}
