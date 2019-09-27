#![cfg(feature = "worklink")]

extern crate rusoto_core;
extern crate rusoto_worklink;

use rusoto_core::Region;
use rusoto_worklink::{ListFleetsRequest, Worklink, WorklinkClient};

#[test]
fn should_list_fleets() {
    let client = WorklinkClient::new(Region::UsEast1);

    let request = ListFleetsRequest::default();

    let result = client.list_fleets(request).sync();
    println!("{:#?}", result);
    result.unwrap();
}
