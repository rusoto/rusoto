#![cfg(feature = "events")]

extern crate rusoto;

use rusoto::events::{CloudWatchEvents, CloudWatchEventsClient, ListRulesRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_rules() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        CloudWatchEventsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListRulesRequest::default();

    client.list_rules(&request).unwrap();
}
