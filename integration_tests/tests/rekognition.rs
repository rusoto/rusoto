#![cfg(feature = "rekognition")]

extern crate rusoto_core;
extern crate rusoto_rekognition;

use rusoto_core::Region;
use rusoto_rekognition::{ListCollectionsRequest, Rekognition, RekognitionClient};

#[tokio::test]
async fn should_list_collections() {
    let client = RekognitionClient::new(Region::UsEast1);
    let request = ListCollectionsRequest::default();

    let result = client.list_collections(request).await.unwrap();
    println!("{:#?}", result);
}
