#![cfg(feature = "events")]

extern crate rusoto_core;
extern crate rusoto_events;

use rusoto_events::{CloudWatchEvents, CloudWatchEventsClient, ListRulesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_rules() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client =
        CloudWatchEventsClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListRulesRequest::default();

    client.list_rules(&request).sync().unwrap();
}
