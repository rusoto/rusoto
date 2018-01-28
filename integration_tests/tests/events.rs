#![cfg(feature = "events")]

extern crate rusoto_core;
extern crate rusoto_events;

use rusoto_events::{CloudWatchEvents, CloudWatchEventsClient, ListRulesRequest};
use rusoto_core::Region;

#[test]
fn should_list_rules() {
    let client = CloudWatchEventsClient::simple(Region::UsEast1);
    let request = ListRulesRequest::default();

    client.list_rules(&request).sync().unwrap();
}
