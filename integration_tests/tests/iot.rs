#![cfg(feature = "iot")]

extern crate env_logger;
extern crate rusoto_core;
extern crate rusoto_iot;

use rusoto_core::Region;
use rusoto_iot::{Iot, IotClient, ListThingsRequest};

#[tokio::test]
async fn should_list_things() {
    let _ = env_logger::try_init();
    let client = IotClient::new(Region::UsEast1);
    let request = ListThingsRequest::default();

    client.list_things(request).await.unwrap();
}
