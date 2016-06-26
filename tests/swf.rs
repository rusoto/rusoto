#![cfg(feature = "swf")]

extern crate rusoto;

use rusoto::swf::{SwfClient, ListDomainsInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_domains() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SwfClient::new(credentials, Region::UsEast1);

    let mut request = ListDomainsInput::default();
    request.maximum_page_size = Some(10);
    request.registration_status = "REGISTERED".to_string();

    match client.list_domains(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
