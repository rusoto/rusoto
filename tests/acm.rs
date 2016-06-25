#![cfg(feature = "acm")]

extern crate rusoto;

use rusoto::acm::{AcmClient, ListCertificatesRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_certificates() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = AcmClient::new(credentials, Region::UsEast1);

    let request = ListCertificatesRequest::default();

    match client.list_certificates(&request) {
    	Ok(response) => {
    		println!("{:#?}", response); 
    		assert!(true)
    	},
    	Err(err) => panic!("Expected OK response, got {:#?}", err)
    };

}


