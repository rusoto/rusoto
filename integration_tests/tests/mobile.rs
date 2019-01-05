#![cfg(feature = "mobile")]

extern crate rusoto_core;
extern crate rusoto_mobile;

use rusoto_core::Region;
use rusoto_mobile::{ListProjectsRequest, Mobile, MobileClient};

#[test]
fn should_list_projects() {
    let client = MobileClient::new(Region::UsEast1);
    let request = ListProjectsRequest::default();

    let result = client.list_projects(request).sync().unwrap();
    println!("Results: {:?}", result);
    assert!(result.projects.is_some());
}
