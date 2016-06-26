#![cfg(feature = "kinesis")]

extern crate rusoto;

use rusoto::kinesis::{KinesisClient, ListStreamsInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_streams() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = KinesisClient::new(credentials, Region::UsEast1);

    let request = ListStreamsInput::default();

    match client.list_streams(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
