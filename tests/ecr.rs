#![cfg(feature = "ecr")]

extern crate rusoto;

use rusoto::ecr::{EcrClient, DescribeRepositoriesRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_repositories() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = EcrClient::new(credentials, Region::UsEast1);

    let request = DescribeRepositoriesRequest::default();

    match client.describe_repositories(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}

