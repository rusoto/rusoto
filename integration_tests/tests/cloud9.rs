#![cfg(feature = "cloud9")]

extern crate rusoto_cloud9;
extern crate rusoto_core;

use rusoto_cloud9::{Cloud9, Cloud9Client, ListEnvironmentsRequest};
use rusoto_core::Region;

#[tokio::test]
async fn should_list_environments() {
    let client = Cloud9Client::new(Region::UsEast1);
    let request = ListEnvironmentsRequest::default();

    let result = client.list_environments(request).await.unwrap();
    println!("{:#?}", result);
}
