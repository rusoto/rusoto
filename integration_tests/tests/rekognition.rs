#![cfg(feature = "rekognition")]

extern crate rusoto_core;
extern crate rusoto_rekognition;

use rusoto_rekognition::{Rekognition, RekognitionClient, ListCollectionsRequest};
use rusoto_core::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_collections() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = RekognitionClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListCollectionsRequest::default();

    let result = client.list_collections(&request).sync().unwrap();
	println!("{:#?}", result);
}