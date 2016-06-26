#![cfg(feature = "events")]

extern crate rusoto;

use rusoto::events::{CloudWatchEventsClient, ListRulesRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_rules() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudWatchEventsClient::new(credentials, Region::UsEast1);

    let request = ListRulesRequest::default();

    match client.list_rules(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
