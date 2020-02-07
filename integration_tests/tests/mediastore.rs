#![cfg(feature = "mediastore")]

extern crate rusoto_core;
extern crate rusoto_mediastore;

use rusoto_core::Region;
use rusoto_mediastore::{ListContainersInput, MediaStore, MediaStoreClient};

#[tokio::test]
async fn should_list_containers() {
    let client = MediaStoreClient::new(Region::UsEast1);
    let request = ListContainersInput::default();

    println!("{:?}", client.list_containers(request).await.unwrap());
}
