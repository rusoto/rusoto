#![cfg(feature = "greengrass")]

extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_greengrass;

use rusoto_core::Region;
use rusoto_greengrass::{GreenGrass, GreenGrassClient, ListGroupsRequest};

#[tokio::test]
async fn should_list_groups() {
    let _ = env_logger::try_init();
    let client = GreenGrassClient::new(Region::UsWest2);

    let request = ListGroupsRequest::default();

    let result = client.list_groups(request).await.unwrap();
    println!("{:#?}", result);
}
