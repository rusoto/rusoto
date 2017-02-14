#![cfg(feature = "rekognition")]
extern crate rusoto;

use rusoto::rekognition::{RekognitionClient, ListCollectionsRequest};
use rusoto::{DefaultCredentialsProvider, Region, default_tls_client};

#[test]
fn should_list_collections() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = RekognitionClient::new(default_tls_client().unwrap(), credentials, Region::UsEast1);
    let request = ListCollectionsRequest::default();

    let result = client.list_collections(&request).unwrap();
	println!("{:#?}", result);
}