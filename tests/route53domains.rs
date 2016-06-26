#![cfg(feature = "route53domains")]

extern crate rusoto;

use rusoto::route53domains::{Route53DomainsClient, ListOperationsRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_operations() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = Route53DomainsClient::new(credentials, Region::UsEast1);

    let request = ListOperationsRequest::default();

    match client.list_operations(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}