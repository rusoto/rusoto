#![cfg(feature = "rekognition")]

extern crate rusoto_core;
extern crate rusoto_rekognition;

use rusoto_rekognition::{Rekognition, RekognitionClient, ListCollectionsRequest};
use rusoto_core::Region;

#[test]
fn should_list_collections() {
    let client = RekognitionClient::simple(Region::UsEast1);
    let request = ListCollectionsRequest::default();

    let result = client.list_collections(&request).sync().unwrap();
	println!("{:#?}", result);
}