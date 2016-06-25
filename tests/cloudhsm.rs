#![cfg(feature = "cloudhsm")]

extern crate rusoto;

use rusoto::cloudhsm::{CloudHsmClient, ListHapgsRequest, ListHsmsRequest, ListLunaClientsRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_hapgs() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmClient::new(credentials, Region::UsEast1);

    let request = ListHapgsRequest::default();

    match client.list_hapgs(&request) {
    	Ok(response) => {
    		println!("{:#?}", response); 
    		assert!(true)
    	},
    	Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}

#[test]
fn should_list_hsms() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmClient::new(credentials, Region::UsEast1);

    let request = ListHsmsRequest::default();

    match client.list_hsms(&request) {
    	Ok(response) => {
    		println!("{:#?}", response); 
    		assert!(true)
    	},
    	Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
#[test]
fn should_list_luna_clients() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CloudHsmClient::new(credentials, Region::UsEast1);

    let request = ListLunaClientsRequest::default();

    match client.list_luna_clients(&request) {
    	Ok(response) => {
    		println!("{:#?}", response); 
    		assert!(true)
    	},
    	Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
