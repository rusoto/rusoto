#![cfg(feature = "opsworks")]

extern crate rusoto;

use rusoto::opsworks::{OpsWorksClient, DescribeStacksRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_describe_stacks() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = OpsWorksClient::new(credentials, Region::UsEast1);

    let request = DescribeStacksRequest::default();

    match client.describe_stacks(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}

#[test]
fn should_describe_my_user_profile() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = OpsWorksClient::new(credentials, Region::UsEast1);

    match client.describe_my_user_profile() {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}