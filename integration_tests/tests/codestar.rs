#![cfg(feature = "codestar")]

extern crate rusoto_core;
extern crate rusoto_codestar;

use rusoto_codestar::{CodeStar, CodeStarClient, ListProjectsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;

#[test]
fn should_list_projects() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodeStarClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListProjectsRequest::default();

    let result = client.list_projects(&request).unwrap();
    println!("Result is {:?}", result);
}
