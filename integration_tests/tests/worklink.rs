#![cfg(feature = "worklink")]

extern crate rusoto_core;
extern crate rusoto_worklink;

use rusoto_core::Region;
use rusoto_worklink::{ListFleetsRequest, Worklink, WorklinkClient};

#[tokio::test]
async fn should_list_fleets() {
    let client = WorklinkClient::new(Region::UsEast1);

    let request = ListFleetsRequest::default();

    let result = client.list_fleets(request).await;
    println!("{:#?}", result);
    result.unwrap();
}
