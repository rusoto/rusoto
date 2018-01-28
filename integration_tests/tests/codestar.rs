#![cfg(feature = "codestar")]

extern crate rusoto_core;
extern crate rusoto_codestar;

use rusoto_codestar::{CodeStar, CodeStarClient, ListProjectsRequest};
use rusoto_core::Region;

#[test]
fn should_list_projects() {
    let client = CodeStarClient::simple(Region::UsEast1);
    let request = ListProjectsRequest::default();

    let result = client.list_projects(&request).sync().unwrap();
    println!("Result is {:?}", result);
}
