#![cfg(feature = "sns")]

extern crate rusoto;

use rusoto::sns::{SnsClient, ListTopicsInput};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_topics() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SnsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListTopicsInput::default();

    let result = client.list_topics(&request).unwrap();
    println!("{:#?}", result);
}


