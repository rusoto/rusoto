#![cfg(feature = "sns")]

extern crate rusoto;

use rusoto::sns::{SnsClient, ListTopicsInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_topics() {
    let client = SnsClient::new(DefaultCredentialsProvider::new().unwrap(), Region::UsEast1);
    let request = ListTopicsInput::default();

    let result = client.list_topics(&request).unwrap();
    println!("{:#?}", result);
}


