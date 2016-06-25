#![cfg(feature = "ds")]

extern crate rusoto;

use rusoto::ds::{DirectoryServiceClient, DescribeTrustsRequest, DescribeDirectoriesRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_trusts() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DirectoryServiceClient::new(credentials, Region::UsEast1);

    let request = DescribeTrustsRequest::default();

    match client.describe_trusts(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}

#[test]
fn should_describe_directories() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = DirectoryServiceClient::new(credentials, Region::UsEast1);

    let request = DescribeDirectoriesRequest::default();

    match client.describe_directories(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}