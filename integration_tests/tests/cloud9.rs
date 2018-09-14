#![cfg(feature = "cloud9")]

extern crate rusoto_core;
extern crate rusoto_cloud9;

use rusoto_cloud9::{Cloud9, Cloud9Client, ListEnvironmentsRequest};
use rusoto_core::Region;

#[test]
fn should_list_environments() {
    let client = Cloud9Client::new(Region::UsEast1);
    let request = ListEnvironmentsRequest::default();

    let result = client.list_environments(request).sync().unwrap();
    println!("{:#?}", result);
}
