#![cfg(feature = "waf-regional")]

extern crate rusoto_core;
extern crate rusoto_waf_regional;

use rusoto_waf_regional::{WAFRegional, WAFRegionalClient, ListRulesRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_rules() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = WAFRegionalClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListRulesRequest::default();

    let result = client.list_rules(&request).unwrap();
    println!("{:#?}", result);
}