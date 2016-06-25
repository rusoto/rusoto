#![cfg(feature = "codecommit")]

extern crate rusoto;

use rusoto::codecommit::{CodeCommitClient, ListRepositoriesInput};
use rusoto::{DefaultCredentialsProvider, Region};

#[test]
fn should_list_repositories() {
    let credentials = DefaultCredentialsProvider::new().unwrap();
    let client = CodeCommitClient::new(credentials, Region::UsEast1);

    let request = ListRepositoriesInput::default();

    match client.list_repositories(&request) {
    	Ok(response) => {
    		println!("{:#?}", response); 
    		assert!(true)
    	},
    	Err(err) => panic!("Expected OK response, got {:#?}", err)
    };
}
