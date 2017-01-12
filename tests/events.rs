#![cfg(feature = "events")]

extern crate rusoto;

use rusoto::events::{CloudWatchEventsClient, ListRulesRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_rules() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudWatchEventsClient::new(credentials, Region::UsEast1).unwrap();
    let request = ListRulesRequest::default();

    client.list_rules(&request).unwrap();
}
