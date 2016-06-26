#![cfg(feature = "storagegateway")]

extern crate rusoto;

use rusoto::storagegateway::{StorageGatewayClient, ListGatewaysInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_gateways() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = StorageGatewayClient::new(credentials, Region::UsEast1);

    let request = ListGatewaysInput::default();

    match client.list_gateways(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
