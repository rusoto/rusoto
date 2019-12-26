#![cfg(feature = "events")]

extern crate rusoto_core;
extern crate rusoto_events;

use rusoto_core::Region;
use rusoto_events::{EventBridge, EventBridgeClient, ListRulesRequest};

#[tokio::test]
async fn should_list_rules() {
    let client = EventBridgeClient::new(Region::UsEast1);
    let request = ListRulesRequest::default();

    client.list_rules(request).await.unwrap();
}
