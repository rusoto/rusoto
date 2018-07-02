#![cfg(feature = "greengrass")]

extern crate rusoto_core;
extern crate rusoto_greengrass;
extern crate env_logger;

use rusoto_greengrass::{GreenGrass, GreenGrassClient, ListGroupsRequest};
use rusoto_core::Region;

#[test]
fn should_list_groups() {
    let _ = env_logger::try_init();
    let client = GreenGrassClient::new(Region::UsWest2);
    
    let request = ListGroupsRequest::default();

    let result = client.list_groups(request).sync().unwrap();
    println!("{:#?}", result);
}