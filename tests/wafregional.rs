#![cfg(feature = "waf-regional")]
extern crate rusoto;

use rusoto::wafregional::{WafRegionalClient, ListRulesRequest};
use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::default_tls_client;

#[test]
fn should_list_rules() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = WafRegionalClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListRulesRequest::default();

    let result = client.list_rules(&request).unwrap();
    println!("{:#?}", result);
}


