#![cfg(feature = "ssm")]

extern crate rusoto;

use rusoto::ssm::{SsmClient, ListDocumentsRequest, ListCommandsRequest, ListCommandInvocationsRequest};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_documents() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SsmClient::new(credentials, Region::UsEast1);

    let request = ListDocumentsRequest::default();

    match client.list_documents(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}

#[test]
fn should_list_commands() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SsmClient::new(credentials, Region::UsEast1);

    let request = ListCommandsRequest::default();

    match client.list_commands(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}

#[test]
fn should_list_command_invocations() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = SsmClient::new(credentials, Region::UsEast1);

    let request = ListCommandInvocationsRequest::default();

    match client.list_command_invocations(&request) {
        Ok(response) => {
            println!("{:#?}", response); 
            assert!(true)            
        },
        Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}